use async_openai::types::CreateCompletionRequestArgs;
use bevy::prelude::*;

use bevy::utils::HashMap;
use bevy_tokio_tasks::TokioTasksRuntime;
use bollard::container::Config;
use bollard::exec::{CreateExecOptions, StartExecResults};

use bollard::image::CreateImageOptions;
use clap::Parser;
use futures_lite::StreamExt;

use bollard::errors::Error;
use bollard::Docker;
use std::fs::{self, OpenOptions};
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::{fs::File, io::BufRead};

// use async_openai::Client;

use reqwest::Client;
use serde::{Deserialize, Serialize};

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

#[derive(Resource)]
struct OpenAIObjects {
    client: Option<Client>,
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
    max_iterations: usize,
    write_file: String,
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
    language: String,
    command: String,
    code: String,
    instructions: String,
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

                let chat_completion =
                    serde_json::from_str::<ChatCompletion>(&response_body).unwrap();

                finish_reason = chat_completion
                    .clone()
                    .choices
                    .first()
                    .unwrap()
                    .finish_reason
                    .clone();

                // let resp = &response.choices.first().unwrap().text;

                // println!("Completions: {:?}", resp);

                local_response = local_response.clone()
                    + &chat_completion.choices.first().unwrap().message.content;
                // finish_reason = response.choices.first().unwrap().finish_reason.clone();

                // if finish_reason == Some("stop".to_string()) {
                //     // println!("Finished Reason: {:?}", finish_reason);
                //     println!("Local response: {}", local_response.clone());
            }
            // let super_local = local_response.clone();
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

fn parse_text(
    mut query: Query<(Entity, &mut ParsingObjects, &mut Unparsed)>,
    mut commands: Commands,
    local_goal: Res<ProjectObjects>,
    settings: Res<Settings>,
) {
    let write_file = settings.write_file.clone();
    for (the_entity, mut object, unparsed) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unparsed>(); // We only want to process the entity once

        match object.as_mut() {
            ParsingObjects::SystemOrientation(_) => match parse_architecture_data(&unparsed.text) {
                Ok(architecture_data) => {
                    append_to_file(&write_file, &architecture_data.clone());
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_tokio_tasks::TokioTasksPlugin::default())
        // .insert_resource(Cmd { cmd: vec![] })
        .insert_resource(ContainerInfo { id: None })
        .insert_resource(Settings {
            max_iterations: 10,
            write_file: "output.json".to_string(),
        })
        .insert_resource(CurrentIteration {
            current_iteration: 0,
        })
        .init_resource::<ProjectObjects>()
        .add_startup_system(prepare_docker_container)
        // .add_startup_system(setup) // will add this back in when I figure out how to load a font
        .add_startup_system(initiate_project)
        // .add_startup_system(setup_openai_client)
        // .add_fixed_timestep(Duration::from_secs(5), "label")
        // .add_fixed_timestep_system("label", 0, print_container_info)
        // .add_system(text_input)
        .add_system(build_prompt)
        .add_system(send_openai_prompt)
        .add_system(parse_text)
        .run();
}
