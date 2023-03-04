use bevy::prelude::*;
use bevy::render::settings;
use bevy::utils::HashMap;
use bevy_tokio_tasks::TokioTasksRuntime;
use bollard::container::Config;
use bollard::errors::Error;
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::image::CreateImageOptions;
use bollard::Docker;
use clap::Parser;
use futures_lite::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::{fs::File, io::BufRead};
use tar::Builder;

#[derive(Resource)]
struct CurrentIteration {
    current_iteration: usize,
}

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = true)]
    debug: bool,
    #[clap(short, long, default_value = "project_goals.txt")]
    project_goals_file: String,
    #[clap(long, default_value = "cool_project_name")]
    project_name: String,
    #[clap(
        long,
        default_value = "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"
    )]
    api_key: String,
}

#[derive(Resource)]
struct ProjectObjects {
    goal: String,
    prompts: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ChatCompletion {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

#[derive(Component)]
struct Unprocessed;

#[derive(Component)]
struct Prompt {
    text: String,
}

#[derive(Component)]
struct Unparsed {
    text: String,
}

#[derive(Component)]
struct Unsent;

#[derive(Resource)]
struct ContainerInfo {
    id: Option<String>,
}

#[derive(Resource)]
struct Settings {
    project_folder: String,
    max_iterations: usize,
    write_file: String,
    project_phase: Phase,
    implementation_finished: bool,
    all_functions: Vec<String>,
    implemented_functions: Vec<String>,
}

enum Phase {
    Planning,
    Implementation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ImplementationDetails {
    result: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TeamLeadContextInput {
    goal: String,
    objects: Vec<String>,
    functions: Vec<String>,
    currentFunction: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TeamLeadContextOutput {
    objects: Vec<String>,
    functions: Vec<String>,
    currentFunction: String,
    description: String,
    testCases: Vec<TestCase>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SystemContext {
    objects: Vec<String>,
    functions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InitialData {
    goal: String,
}

#[derive(Clone, Debug, Component)]
enum ParsingObjects {
    SystemOrientation(InitialData),
    Architecture(SystemContext),
    MakeTicket(TeamLeadContextInput),
    CompletedTicket(TeamLeadContextOutput),
    Implementation(Code),
}

#[derive(Clone, Debug, Component)]
enum TerminalInteraction {
    Input(String),
    Output(String),
    Err(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestCase {
    input: String,
    output: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Code {
    filename: String,
    currentFunction: String,
    language: String,
    command: String,
    code: String,
    instructions: String,
}

#[derive(Component)]
struct InitiateImplementation;

#[derive(Resource)]
struct RuntimeSettings {
    goal: Option<String>,
    files: Option<Vec<String>>,
    project_phase: Phase,
    terminal_session: Option<Vec<TerminalInfo>>,
    project_progress: Option<Vec<ProjectObjects>>,
 }

enum TerminalInfo {
    Input(String),
    Output(String),
    Err(String),
}

fn file_exists(file_name: &str) -> bool {
    let path = Path::new(file_name);
    path.is_file()
}

fn create_tarball(file_names: Vec<String>) -> std::io::Result<()> {
    // Create a new tar archive
    let file = File::create("archive.tar")?;
    let mut builder = Builder::new(file);

    // Add files to the archive
    for file_name in file_names {
        if !file_exists(&file_name) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            ));
        }
        builder.append_file(file_name.clone(), &mut File::open(file_name)?)?;
    }

    // Finish the archive
    builder.finish()?;

    Ok(())
}

fn parse_architecture_data(input: &str) -> serde_json::Result<SystemContext> {
    println!("Parsing Architecture Data:\n {}", input);
    serde_json::from_str(input).map_err(|e| e.into())
}

fn parse_ticket_data(input: &str) -> serde_json::Result<TeamLeadContextOutput> {
    println!("Parsing Ticket Data:\n {}", input);
    serde_json::from_str(input).map_err(|e| e.into())
}

fn parse_implementation_data(input: &str) -> serde_json::Result<Code> {
    println!("Parsing Ticket Data:\n {}", input);
    serde_json::from_str(input).map_err(|e| e.into())
}

fn append_to_file<T: Serialize + Deserialize<'static>>(
    filename: &str,
    data: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    let mut writer = BufWriter::new(file);

    // Serialize data to JSON and write to file
    let serialized_data = serde_json::to_string(data)?;
    writeln!(writer, "{}", serialized_data)?;

    Ok(())
}

// Implementing this trait allows us to create a resource that is accessible from all future systems that we create.
impl FromWorld for ProjectObjects {
    fn from_world(_world: &mut World) -> Self {
        let args = Args::parse();
        let mut goal = String::new();
        match File::open(args.project_goals_file) {
            Ok(file) => {
                // read the file
                let reader = std::io::BufReader::new(file);
                println!("Project Goals Found!");
                for line in reader.lines() {
                    //add line to goal
                    goal.push_str(&line.unwrap().clone());
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                println!("Make sure that the 'project_goals_file' exists and is in the same directory as the executable.The default location is 'project_goals.txt")
            }
        }
        let prompts = load_prompts();
        Self { goal, prompts }
    }
}

#[cfg(unix)]
fn new_docker() -> Result<Docker, Error> {
    Docker::connect_with_socket_defaults()
}

#[cfg(not(unix))]
fn new_docker() -> Result<Docker> {
    Docker::new("tcp://127.0.0.1:8080")
}

fn prepare_docker_container(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(|mut ctx| async move {
        let docker = new_docker().unwrap();
        let image = "ubuntu:latest";
        docker
            .create_image(
                Some(CreateImageOptions {
                    from_image: image,
                    ..Default::default()
                }),
                None,
                None,
            )
            .next()
            .await;

        let image_config = Config {
            image: Some(image),
            tty: Some(true),
            attach_stdin: Some(true),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            open_stdin: Some(true),
            ..Default::default()
        };

        let id = docker
            .create_container::<&str, &str>(None, image_config.clone().into())
            .await
            .unwrap()
            .id;

        ctx.run_on_main_thread(move |ctx| {
            ctx.world.insert_resource(ContainerInfo {
                id: Some(id.clone()),
            });
        })
        .await;
    });
}

fn send_docker_command(
    project_object: Res<ProjectObjects>,
    container_info: Res<ContainerInfo>,
    mut runtime: ResMut<TokioTasksRuntime>,
    commands: Commands,
) {
    // let local_cmd = cmd.cmd.pop().unwrap().unwrap();

    let id = container_info.id.clone().unwrap();

    runtime.spawn_background_task(|mut ctx| async move {
        let docker = new_docker().unwrap();
        // println!("Docker Container: {:?}", &docker);

        docker.start_container::<String>(&id, None).await;

        let exec = docker
            .create_exec(
                &id,
                CreateExecOptions {
                    attach_stdout: Some(true),
                    attach_stdin: Some(true),
                    privileged: Some(true),
                    tty: Some(true),
                    attach_stderr: Some(true),
                    cmd: Some(vec!["/bin/bash".to_string(), "-c".to_string()]),
                    ..Default::default()
                },
            )
            .await
            .unwrap()
            .id;
        if let Ok(StartExecResults::Attached { mut output, .. }) =
            docker.start_exec(&exec, None).await
        {
            while let Some(Ok(msg)) = output.next().await {
                print!("Message: {}", msg);
            }
        } else {
            unreachable!();
        }
    });
}

fn load_prompts() -> HashMap<String, String> {
    let directory_path = "./src/prompts";
    let mut file_map = HashMap::new();
    let directory = Path::new(directory_path);

    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if file_path.is_file() {
            if let Some(file_name) = file_path.clone().file_name().and_then(|n| n.to_str()) {
                if let Some(file_stem) = Path::new(file_name).file_stem().and_then(|s| s.to_str()) {
                    let file_contents = fs::read_to_string(file_path).unwrap();
                    file_map.insert(file_stem.to_string(), file_contents);
                }
            }
        }
    }

    file_map
}

fn initiate_project(goal: Res<ProjectObjects>, mut commands: Commands) {
    println!("Project Goals: \n------------------\n");
    println!("{}", goal.goal);
    println!("\n------------------\n");

    // cmd.cmd.push(Some(ParsingObjects::SystemOrientation(
    //     InitialData { goal: goal.goal.clone() })));

    commands.spawn((
        ParsingObjects::SystemOrientation(InitialData {
            goal: goal.goal.clone(),
        }),
        Unprocessed,
    ));
}

fn build_prompt(
    project_object: Res<ProjectObjects>,
    settings: ResMut<Settings>,
    mut current_iteration: ResMut<CurrentIteration>,
    mut query: Query<(Entity, &mut ParsingObjects, &mut Unprocessed)>,
    mut commands: Commands,
) {
    for (the_entity, mut object, _unprocessed) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unprocessed>(); // We only want to process the entity once

        print!(
            "Sending OpenAI Command: {:?}\nCurrent iteration: {:?}\n",
            &object, &current_iteration.current_iteration
        );

        current_iteration.current_iteration += 1;

        if current_iteration.current_iteration > settings.max_iterations {
            println!("Max iterations reached");
            return;
        }

        // here is where we determine the prompt based on the stage of development
        let mut prompt = String::new();

        println!(
            "project_object prompt keys: {:?}",
            project_object.prompts.keys()
        );

        // let local_setting = settings.stage.clone();

        match object.as_mut() {
            ParsingObjects::SystemOrientation(initial_data) => {
                prompt = project_object
                    .prompts
                    .get("softwareArchitect")
                    .unwrap()
                    .clone()
                    .to_string();
                prompt = prompt + &serde_json::to_string(&initial_data.clone()).unwrap();
            }
            ParsingObjects::Architecture(_) => {
                todo!()
            }
            ParsingObjects::MakeTicket(ticket_context) => {
                prompt = project_object
                    .prompts
                    .get("teamLead")
                    .unwrap()
                    .clone()
                    .to_string();
                prompt = prompt + &serde_json::to_string(&ticket_context).unwrap();
            }
            ParsingObjects::CompletedTicket(ticket) => {
                prompt = project_object
                    .prompts
                    .get("developers")
                    .unwrap()
                    .clone()
                    .to_string();
                prompt = prompt + &serde_json::to_string(&ticket).unwrap();
            }
            ParsingObjects::Implementation(_) => todo!(),
        };

        commands
            .entity(the_entity)
            .insert(Prompt { text: prompt })
            .insert(Unsent);
    }
}

fn send_openai_prompt(
    // openai: Res<OpenAIObjects>,
    runtime: ResMut<TokioTasksRuntime>,
    mut query: Query<(Entity, &ParsingObjects, &mut Prompt, &mut Unsent)>,
    mut commands: Commands,
) {
    for (the_entity, _object, mut prompt, _unsent) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unsent>(); // We only want to process the entity once
                                                        // let client = openai.client.clone().unwrap();
        let local_prompt = prompt.as_mut().text.clone();

        let args = Args::parse();
        let api_key = args.api_key.clone();

        runtime.spawn_background_task(move |mut ctx| async move {
            let mut finish_reason = "".to_string();

            let mut local_response = String::new();
            while finish_reason != "stop" {
                let full_string = local_prompt.clone() + &local_response.clone();
                // let request = CreateCompletionRequestArgs::default()
                //     .model("text-davinci-003")
                //     .prompt(&full_string)
                //     .max_tokens(200_u16)
                //     .build()
                //     .unwrap();

                // let response = client
                //     .completions() // Get the API "group" (completions, images, etc.) from the client
                //     .create(request) // Make the API call in that "group"
                //     .await
                //     .unwrap();

                let url = "https://api.openai.com/v1/chat/completions";

                let client = Client::new();
                let request_body = ChatCompletionRequest {
                    model: "gpt-3.5-turbo".to_string(),
                    messages: vec![Message {
                        role: "user".to_string(),
                        content: full_string.clone(),
                    }],
                };

                let response = client
                    .post(url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", api_key))
                    .json(&request_body)
                    .send()
                    .await
                    .unwrap();
                println!("{:?}", response);

                let response_body = response.text().await.unwrap();

                println!("\n{:?}", response_body.clone());

                let chat_completion: ChatCompletion;

                match serde_json::from_str::<ChatCompletion>(&response_body) {
                    Ok(local_chat_completion) => {
                        println!("Chat Completion: {:?}", local_chat_completion);
                        chat_completion = local_chat_completion;

                        finish_reason = chat_completion
                            .clone()
                            .choices
                            .first()
                            .unwrap()
                            .finish_reason
                            .clone();

                        local_response = local_response.clone()
                            + &chat_completion.choices.first().unwrap().message.content;
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }

                //     // println!("Finished Reason: {:?}", finish_reason);
                //     println!("Local response: {}", local_response.clone());
            }

            let super_local = local_response.clone();

            ctx.run_on_main_thread(move |mut ctx| {
                ctx.world
                    .entity_mut(the_entity.clone())
                    .insert(Unparsed { text: super_local });
            })
            .await;

            // }
            // }
        });
    }
}

fn contains_mostly_similar_strings(v1: &Vec<String>, v2: &Vec<String>) -> bool {
    // Make copies of both vectors so we can modify them safely.

    if v1.len() != v2.len() {
        // doesn't even contain the same number of strings
        return false;
    }

    let mut a = v1.clone();
    let mut b = v2.clone();

    // make all strings in a lowercase
    for i in 0..a.len() {
        a[i] = a[i].to_lowercase();
    }

    //same for b
    for i in 0..b.len() {
        b[i] = b[i].to_lowercase();
    }

    // Sort the vectors so we can compare them element-wise.
    a.sort();
    b.sort();

    println!("comparing {:?} to {:?}", a, b);

    let mut passes: Vec<bool> = Vec::new();
    // Check if there's any difference between the sorted vectors.
    for i in 0..a.len() {
        for j in 0..b.len() {
            let threshold: usize = (a[i].len() + b[j].len()) / 2 / 3; // 33% of the average length of the two strings
            if levenshtein_distance(&a[i], &b[j]) <= threshold {
                passes.push(true);
                // break out of the inner loop
                break;
            }
        }
    }

    if passes.len() == a.len() {
        return true;
    } else {
        return false;
    }
}

fn levenshtein_distance(s: &str, t: &str) -> usize {
    let n = s.chars().count();
    let m = t.chars().count();

    if n == 0 || m == 0 {
        return n + m;
    }

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..=n {
        dp[i][0] = i;
    }
    for j in 0..=m {
        dp[0][j] = j;
    }

    for (i, sc) in s.chars().enumerate() {
        for (j, tc) in t.chars().enumerate() {
            let cost = if sc == tc { 0 } else { 1 };

            dp[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(dp[i][j + 1] + 1, dp[i + 1][j] + 1),
                dp[i][j] + cost,
            );
        }
    }

    dp[n][m]
}

pub fn get_function_names(v: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for s in v {
        let first_bracket_index = match s.find('(') {
            Some(index) => index,
            None => continue,
        };
        let function_name: String = s[..first_bracket_index].to_string();

        result.push(function_name.to_string());
    }
    result
}

fn parse_text(
    mut query: Query<(Entity, &mut ParsingObjects, &mut Unparsed)>,
    mut commands: Commands,
    local_goal: Res<ProjectObjects>,
    mut docker_terminal : ResMut<RuntimeSettings>,
    mut settings: ResMut<Settings>,
) {
    let write_file = settings.write_file.clone();
    for (the_entity, mut object, unparsed) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unparsed>(); // We only want to process the entity once

        match object.as_mut() {
            ParsingObjects::SystemOrientation(_) => match parse_architecture_data(&unparsed.text) {
                Ok(architecture_data) => {
                    append_to_file(&write_file, &architecture_data.clone());
                    settings.all_functions =
                        get_function_names(&architecture_data.functions.clone());
                    println!("All functions: {:?}", settings.all_functions.clone());
                    for function in &architecture_data.functions {
                        let mut ticket = TeamLeadContextInput {
                            goal: local_goal.goal.clone(),
                            functions: architecture_data.functions.clone(),
                            currentFunction: function.clone(),
                            objects: architecture_data.objects.clone(),
                        };

                        commands.spawn((ParsingObjects::MakeTicket(ticket), Unprocessed));
                        // return Ok(ParsingObjects::Architecture(architecture_data))
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    let mut prompt = "Given an input and an error, please output well formatted json that fixes the error.
                ".to_string();
                    prompt = prompt + &unparsed.text.clone();
                    prompt = prompt + e.to_string().as_str();

                    commands
                        .entity(the_entity)
                        .insert(Prompt { text: prompt })
                        .insert(Unsent);
                }
            },
            ParsingObjects::Architecture(_) => todo!(),
            ParsingObjects::MakeTicket(_) => match parse_ticket_data(&unparsed.text) {
                Ok(ticket_data) => {
                    append_to_file(&write_file, &ticket_data.clone());

                    commands.spawn((ParsingObjects::CompletedTicket(ticket_data), Unprocessed));
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    let mut prompt = "Given an input and an error, please output well formatted json that fixes the error.
                    ".to_string();
                    prompt = prompt + &unparsed.text.clone();
                    prompt = prompt + e.to_string().as_str();

                    commands
                        .entity(the_entity)
                        .insert(Prompt { text: prompt })
                        .insert(Unsent);
                }
            },
            ParsingObjects::CompletedTicket(_) => {
                let json = parse_implementation_data(&unparsed.text);
                match json {
                    Ok(code) => {
                        append_to_file(&write_file, &code.clone());
                        settings
                            .implemented_functions
                            .push(code.currentFunction.clone());

                        let function_names: Vec<String> =
                            get_function_names(&settings.implemented_functions);

                        if contains_mostly_similar_strings(&function_names, &settings.all_functions)
                        {
                            println!("All functions implemented!");
                            settings.implementation_finished = true;
                            
                            commands.entity(the_entity).insert(InitiateImplementation);
                        }
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        let mut prompt = "Given an input and an error, please output well formatted json that fixes the error.
                        ".to_string();
                        prompt = prompt + &unparsed.text.clone();
                        prompt = prompt + e.to_string().as_str();

                        commands
                            .entity(the_entity)
                            .insert(Prompt { text: prompt })
                            .insert(Unsent);
                    }
                }
            }
            ParsingObjects::Implementation(_) => todo!(),
        };
    }
}

fn initiate_implementation(settings: ResMut<Settings>, mut docker_terminal : ResMut<RuntimeSettings>) {
    if settings.implementation_finished {
        println!("Starting implementation");
        docker_terminal.goal = Some(settings.goal.clone());
        docker_terminal.files = Some(settings.implemented_functions.clone());
    }
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_tokio_tasks::TokioTasksPlugin::default())
        // .insert_resource(Cmd { cmd: vec![] })
        .insert_resource(ContainerInfo { id: None })
        .insert_resource(RuntimeSettings{
            goal: None,
            files: None,
            project_progress: None,
            terminal_session: None,
            project_phase: Phase::Planning,
           })
        .insert_resource(Settings {
            max_iterations: 10,
            write_file: "output.json".to_string(),
            implementation_finished: false,
            project_phase: Phase::Planning,
            all_functions: vec![],
            implemented_functions: vec![],
            project_folder: "project".to_string(),
        })
        .insert_resource(CurrentIteration {
            current_iteration: 0,
        })
        .init_resource::<ProjectObjects>()
        .add_startup_system(prepare_docker_container)
        .add_startup_system(initiate_project)
        .add_system(build_prompt)
        .add_system(send_openai_prompt)
        .add_system(parse_text)
        .add_system(initiate_implementation)
        .run();
}
