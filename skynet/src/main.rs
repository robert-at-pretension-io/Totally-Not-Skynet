use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_tokio_tasks::TokioTasksRuntime;
use bollard::container::Config;
use bollard::errors::Error;
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::image::CreateImageOptions;
use bollard::Docker;
use clap::Parser;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serpapi_search_rust::serp_api_search::SerpApiSearch;
use std::fmt::{self};
use std::sync::{Arc, Mutex};
use std::vec;
use std::{fs::File, io::BufRead};
use tokio::net::{TcpListener, TcpStream};

//import bevy hashmap

mod helper_functions;

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
    api_key_openai: String,
    #[clap(
        long,
        default_value = "a602132fb4dd2bad19d4df9532f26aa36d8bfadd8b08311f5fd96db7178b261c"
    )]
    api_key_serp: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Process {
    name: String,
    triggered_by: String,
    triggers: String,
    steps: Vec<String>,
    description: String,
    creates_process_branch: bool,
    waits_for_branch_completion: bool,
}

// struct ProcessRuntime {
//     process: Process,
//     id: String,
// }

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

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    name: String,
    system: String,
    prompt: String,
}
impl Action {
    fn new(file_contents: String) -> Option<Action> {
        // deserialize the file contents
        match serde_json::from_str(&file_contents) {
            Ok(action) => Some(action),
            Err(e) => {
                println!("Error parsing action file: {}", e);
                None
            }
        }
    }
}

impl Process {
    fn new(file_contents: String) -> Option<Process> {
        // deserialize the file contents
        match serde_json::from_str(&file_contents) {
            Ok(action) => Some(action),
            Err(e) => {
                println!("Error parsing action file: {}", e);
                None
            }
        }
    }
}

#[derive(Component)]
struct Unparsed {
    text: String,
}

#[derive(Component)]
struct Unsent;

#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    role: Role,
    content: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Copy)]
enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl fmt::Display for Role {
    // this is the implementation of the fmt::Display trait
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
        }
    }
}

#[derive(Component)]
struct InitiateImplementation;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum ViewMode {
    Action,
    Process,
}

impl ViewMode {
    fn toggle(&mut self) {
        match self {
            ViewMode::Action => *self = ViewMode::Process,
            ViewMode::Process => *self = ViewMode::Action,
        }
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct NodeGraph {
//     nodes: Vec<Node>,
//     edges: Vec<Edge>,
// }

#[derive(Resource, Clone)]
struct RuntimeSettings {
    goal: Option<String>,
    // node_graphs: Option<HashMap<ViewMode, NodeGraph>>,
    max_iterations: usize,
    write_file: String,
    view_mode: ViewMode,
    available_actions: Vec<String>,
    actions: Option<Vec<Action>>,
    processes: Option<Vec<Process>>,
    implemented_thus_far: Option<Vec<Code>>,
    current_action: Option<Action>,
    current_prompt: Option<String>,
    recording_in_progress: bool,
    container_id: Option<String>,
    current_iteration: usize,
    log: Option<Vec<Message>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Code {
    code: Option<String>,
    language: Option<String>,
    description: Option<String>,
}

impl fmt::Display for RuntimeSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Current Iteration:\t{}", self.current_iteration)?;
        writeln!(f, "avilable actions:\t{:?}", self.available_actions)?;
        writeln!(
            f,
            "Current action:\t\t[{}]:[{}]",
            self.current_action.as_ref().unwrap().name,
            self.current_action.as_ref().unwrap().system
        )?;
        writeln!(f, "Log:")?;

        // only print the last log entry:
        if let Some(log) = self.log.as_ref() {
            // Write out all of the log entries:
            for entry in log {
                writeln!(f, "\t[{}]:[{}]", entry.role, entry.content)?;
            }
        }
        Ok(())
    }
}

async fn get_search_results() {
    // read secret api key from environment variable
    // To get the key simply copy/paste from https://serpapi.com/dashboard.
    let _params = HashMap::<String, String>::new();

    let args = Args::parse();

    let api_key = args.api_key_serp;

    println!("let's search about coffee on google");
    let mut params: std::collections::HashMap<String, String> =
        std::collections::HashMap::<String, String>::new();
    params.insert("q".to_string(), "coffee".to_string());
    params.insert(
        "location".to_string(),
        "Austin, TX, Texas, United States".to_string(),
    );

    // initialize the search engine
    let _search = SerpApiSearch::google(params, api_key);

    // search returns a JSON as serde_json::Value which can be accessed like a HashMap.
    println!("waiting...");
    // let results = search.getJson(params).await.unwrap();
    // let organic_results = results["organic_results"].as_array().unwrap();
    // println!("results received");
    // println!("--- JSON ---");
    // println!(" - number of organic results: {}", organic_results.len());
    // println!(" - organic_results first result description: {}", results["organic_results"][0]["about_this_result"]["source"]["description"]);
    // let places = results["local_results"]["places"].as_array().unwrap();
    // println!("number of local_results: {}", places.len());
    // println!(" - local_results first address: {}", places[0]["address"]);

    // // search returns text
    // println!("--- HTML search ---");
    // let raw = search.html().await.unwrap();
    // print!(" - raw HTML size {} bytes\n", raw.len());
    // print!(" - async search completed with {}\n", results["search_parameters"]["engine"]);
    // print!("ok");
}

use cpal::traits::DeviceTrait;
use cpal::{FromSample, Sample};
use std::io::BufWriter;

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        hound::SampleFormat::Float
    } else {
        hound::SampleFormat::Int
    }
}

fn wav_spec_from_config(config: cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle, tx: crossbeam_channel::Sender<U>)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                tx.send(sample.clone()).unwrap();
                writer.write_sample(sample).ok();
            }
        }
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

// fn ui_example_system(
//     mut egui_ctx: Query<&mut EguiContext>,
//     mut runtime_settings: ResMut<RuntimeSettings>,
// ) {
//     let mut ctx = match egui_ctx.get_single_mut().ok() {
//         Some(ctx) => ctx,
//         None => {
//             return;
//         }
//     };

//     egui::SidePanel::left("side_panel").show(ctx.get_mut(), |ui| {
//         ui.heading("Side Panel");
//         ui.label("This is a side panel");

//         if ui.button("Change View").clicked() {
//             runtime_settings.view_mode.toggle();
//         }
//     });

//     egui::Window::new("Second Window")
//         .vscroll(true)
//         .show(ctx.get_mut(), |ui| {
//             ui.horizontal(|ui| {
//                 ui.label("Write something else: ");
//             });
//         });
// }

fn prepare_docker_container(
    runtime: Res<TokioTasksRuntime>,
    _runtime_settings: ResMut<RuntimeSettings>,
) {
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
            ctx.world
                .get_resource_mut::<RuntimeSettings>()
                .unwrap()
                .container_id = Some(id);
        })
        .await;
    });
}

fn send_docker_command(
    runtime_settings: ResMut<RuntimeSettings>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    let id = runtime_settings.container_id.clone().unwrap();

    runtime.spawn_background_task(|_ctx| async move {
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

fn initiate_project(mut runtime_settings: ResMut<RuntimeSettings>, mut commands: Commands) {
    let args = Args::parse();
    let mut goal = String::new();

    // retrieve the goal from the project goal file
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
    let actions = helper_functions::load_actions(&"./src/actions");

    //collect all of the actions into a vector of strings
    let mut my_actions = Vec::new();
    for action in actions.iter() {
        my_actions.push(action.name.clone());
    }

    // let processes = helper_functions::load_processes(&"./src/processes", my_actions);

    runtime_settings.goal = Some(goal.clone());
    // loop through the actions and add the possible actions to the possible action list
    for action in actions.iter() {
        let action = &action.name;
        runtime_settings.available_actions.push(action.clone());
    }

    runtime_settings.actions = Some(actions.clone());
    // runtime_settings.processes = Some(processes.clone());

    let log_entry = format!("Goal: {}", goal.clone());

    let message: Message = Message {
        content: log_entry.clone(),
        role: Role::User,
    };

    runtime_settings.log = Some(vec![message]);
    println!("Project Goals: \n------------------\n");
    println!("{}", goal);
    println!("\n------------------\n");

    commands.spawn((Unprocessed,));
}

fn build_message_log(
    mut runtime_settings: ResMut<RuntimeSettings>,
    mut query: Query<(Entity, &mut Unprocessed)>,
    mut commands: Commands,
) {
    let mut current_iteration = runtime_settings.current_iteration.clone();
    for (the_entity, _unprocessed) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unprocessed>(); // We only want to process the entity once

        print!("\nCurrent iteration: {:?}\n", &current_iteration);

        current_iteration += 1;
        runtime_settings.current_iteration = current_iteration;

        if current_iteration > runtime_settings.max_iterations {
            println!("Max iterations reached");
            return;
        }

        // here is where we determine the prompt based on the stage of development
        let mut prompt = String::new();

        let mut system: String = String::new();

        // At the very least, the goal should be here
        let mut log: Vec<Message> = runtime_settings.log.clone().unwrap();

        let actions = runtime_settings.actions.clone().unwrap();
        // check to see if the current action is equal to "choose_action"
        let current_action = runtime_settings.current_action.clone();

        // if this is the first time through the loop
        if current_action.is_none() || current_action.clone().unwrap().name == "choose_action" {
            // get the action where the action is "choose_action"
            let current_action = actions
                .iter()
                .find(|&action| action.name == "choose_action")
                .unwrap();

            prompt = current_action.prompt.clone();
            system = current_action.system.clone();

            runtime_settings.current_action = Some(current_action.clone());

            // The prompt consists of the log appended to the top of the prompt for the current action

            // make the action : description pairs
            for action in actions.iter() {
                prompt = prompt + &format!("\n{} : {}", action.name, action.system);
            }

            prompt = prompt + &format!("\nAction to take:");
        } else {
            prompt = current_action.clone().unwrap().prompt.clone();
            system = current_action.clone().unwrap().system.clone();
        }

        // insert the prompt at the end of the log
        let prompt_message: Message = Message {
            content: prompt.clone(),
            role: Role::User,
        };

        log.push(prompt_message);

        if current_action.clone().is_some() {
            // if the first entry in the log is system then change the content:
            if log[0].role == Role::System {
                log[0].content = format!("{}", system);
            }
            // otherwise, we need to add the system to the beginning of the log
            else {
                let system_message: Message = Message {
                    content: format!("{}", system),
                    role: Role::System,
                };
                log.insert(0, system_message);
            }

            let action = current_action.clone().unwrap();
            println!(
                "sending to open ai: [{}] : [{}]",
                &action.name, &action.system
            );
        }

        runtime_settings.current_prompt = Some(prompt.clone());
        runtime_settings.log = Some(log.clone());

        println!("The current log: \n{:#?}", log.clone());

        commands.entity(the_entity).insert(Unsent);
    }
}

fn send_openai_prompt(
    // openai: Res<OpenAIObjects>,
    runtime: ResMut<TokioTasksRuntime>,
    runtime_settings: Res<RuntimeSettings>,
    mut query: Query<(Entity, &mut Unsent)>,
    mut commands: Commands,
) {
    for (the_entity, _unsent) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unsent>(); // We only want to process the entity once

        println!("runtime settings:\n {}", runtime_settings.clone());

        let log = runtime_settings.log.clone().unwrap();
        let args = Args::parse();
        let api_key = args.api_key_openai.clone();

        runtime.spawn_background_task(move |mut ctx| async move {
            let mut finish_reason = "".to_string();

            let mut local_response = String::new();
            while finish_reason != "stop" {
                let url = "https://api.openai.com/v1/chat/completions";

                let client = Client::new();
                let request_body = ChatCompletionRequest {
                    model: "gpt-3.5-turbo".to_string(),
                    messages: log.clone(),
                };

                let response = client
                    .post(url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", api_key))
                    .json(&request_body)
                    .send()
                    .await
                    .unwrap();

                let response_body = response.text().await.unwrap();

                let chat_completion: ChatCompletion;

                match serde_json::from_str::<ChatCompletion>(&response_body) {
                    Ok(local_chat_completion) => {
                        // println!("Chat Completion: {:?}", local_chat_completion);
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
                        println!("Error: {:?}\nResponse Body: {:?}", e, response_body.clone());
                    }
                }
            }

            let super_local = local_response.clone();
            println!("\nResponse: {:?}\n", super_local.clone());

            ctx.run_on_main_thread(move |ctx| {
                ctx.world.entity_mut(the_entity.clone()).insert(Unparsed {
                    text: super_local.clone(),
                });
            })
            .await;
        });
    }
}

use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

fn start_websocket_server(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(move |ctx| async {
        let mut listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

        while let Ok((stream, addr)) = listener.accept().await {
            let ws_stream = tokio_tungstenite::accept_async(stream)
                .await
                .expect("Error during the websocket handshake occurred");
            println!("WebSocket connection established: {}", addr);

            let (outgoing, incoming) = ws_stream.split();

            incoming.try_for_each( |msg| async move {
                println!("Received a message from {}: {}", addr, &msg.to_text().unwrap());
                Ok(())
            }).await;
        }
    });
}

fn process_text(
    mut query: Query<(Entity, &mut Unparsed)>,
    mut commands: Commands,
    mut runtime_settings: ResMut<RuntimeSettings>,
) {
    let write_file = runtime_settings.write_file.clone();
    for (the_entity, unparsed) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unparsed>(); // We only want to process the entity once

        // write the unparsed text to the log:
        use std::fs::OpenOptions;
        use std::io::prelude::*;

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(write_file.clone())
            .unwrap();

        let debug_string = format!(
            "\n--------------\n{}\n--------------\n",
            runtime_settings.clone()
        );

        file.write_all(debug_string.clone().as_bytes()).unwrap();

        // Make an assistant response

        let new_message: Message = Message {
            content: unparsed.text.clone(),
            role: Role::Assistant,
        };

        runtime_settings
            .log
            .as_mut()
            .unwrap()
            .push(new_message.clone());

        // before going back into the prompt creation loop, we need to determine if the initialization agent has given a valid response. That is, it must be one of the actions in the list of actions.
        let available_actions = runtime_settings.available_actions.clone();

        // if the last action was 'choose_action' then we need to loop through the available actions and see if the response is one of them

        let current_action = runtime_settings.current_action.clone().unwrap();
        if current_action.name == "choose_action" {
            let mut valid_response = false;
            for action in available_actions.iter() {
                if unparsed.text.contains(action) {
                    valid_response = true;
                    // get the action corresponding to the action
                    let actions = runtime_settings.actions.clone().unwrap();
                    let current_action = actions
                        .iter()
                        .find(|&this_action| this_action.name.to_string() == action.to_string())
                        .unwrap();
                    runtime_settings.current_action = Some(current_action.clone());
                }
            }

            if !valid_response {
                let actions = runtime_settings.actions.clone().unwrap();
                let current_action = actions
                    .iter()
                    .find(|&action| action.name == "choose_action")
                    .unwrap();

                runtime_settings.current_action = Some(current_action.clone());

                let _log = runtime_settings.log.clone().unwrap();

                // come up with a message that collects all of the available actions
                let mut available_actions_string = "".to_string();
                for action in runtime_settings.available_actions.clone().iter() {
                    available_actions_string =
                        available_actions_string.clone() + &action.to_string() + ", ";
                }

                let content = "Sorry, Please select one of the following: ".to_string()
                    + &available_actions_string;

                let new_message: Message = Message {
                    content: content,
                    role: Role::User,
                };

                runtime_settings
                    .log
                    .as_mut()
                    .unwrap()
                    .push(new_message.clone());
                commands.entity(the_entity).insert(Unsent);
                return;
            }
        } else {
            let actions = runtime_settings.actions.clone().unwrap();
            let current_action = actions
                .iter()
                .find(|&action| action.name == "choose_action")
                .unwrap();

            // get the current action, if it was a developer then we will attempt to parse the code blocks
            if runtime_settings.current_action.clone().unwrap().name == "developer" {
                let code_blocks = parse_code_blocks(&unparsed.text.clone());
                for code_block in code_blocks.iter() {
                    let _language = code_block.get("language").unwrap();
                    let code = code_block.get("code").unwrap();

                    let actions = runtime_settings.actions.clone().unwrap();

                    let current_action = actions
                        .iter()
                        .find(|&action| action.name == "code_description")
                        .unwrap();

                    let prompt = current_action.prompt.clone();

                    // get the prompt, system message, ect from action information

                    // code_description

                    let content = prompt.clone() + &code.clone();

                    let new_message: Message = Message {
                        content: content,
                        role: Role::User,
                    };

                    runtime_settings
                        .log
                        .as_mut()
                        .unwrap()
                        .push(new_message.clone());
                }
            }

            runtime_settings.current_action = Some(current_action.clone());
        }

        commands.entity(the_entity).insert(Unprocessed);
    }
}

use regex::Regex;

fn parse_code_blocks(text: &str) -> Vec<HashMap<String, String>> {
    let re = Regex::new(r"\[code:(?P<language>[\w]+)\](?P<code>.*?)\[/code\]").unwrap();
    let mut code_blocks = Vec::new();

    for captures in re.captures_iter(text) {
        let mut code_block = HashMap::new();
        code_block.insert("language".to_string(), captures["language"].to_string());
        code_block.insert("code".to_string(), captures["code"].to_string());
        code_blocks.push(code_block);
    }

    code_blocks
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    _runtime: Res<TokioTasksRuntime>,
    _runtime_settings: ResMut<RuntimeSettings>,
) {
    if keys.just_pressed(KeyCode::Space) {

        //     // Space was just pressed
        //     if !runtime_settings.recording_in_progress {
        //         runtime.spawn_background_task(|ctx| async move {
        //             record_audio(ctx).await;
        //         });
        //         println!("Space was just pressed -- recording audio");
        //         runtime_settings.recording_in_progress = true;
        //     } else {
        //         println!("Space was just pressed -- stopping recording audio");
        //         runtime_settings.recording_in_progress = false;
        //     }
    }
    // if keys.just_released(KeyCode::LControl) {
    //     // Left Ctrl was released
    // }
    if keys.pressed(KeyCode::W) {
        // W is being held down
    }

    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
        // Either the left or right shift are being held down
    }
    if keys.any_just_pressed([KeyCode::Delete, KeyCode::Back]) {
        // Either delete or backspace was just pressed
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_tokio_tasks::TokioTasksPlugin::default())
        // .add_plugin(EguiPlugin)
        .insert_resource(RuntimeSettings {
            goal: None,
            view_mode: ViewMode::Process,
            available_actions: vec![],
            current_action: None,
            current_prompt: None,
            log: None,
            container_id: None,
            recording_in_progress: false,
            actions: None,
            processes: None,
            current_iteration: 1,
            implemented_thus_far: None,
            max_iterations: 3,
            write_file: "output.txt".to_string(),
        })
        .add_startup_system(prepare_docker_container)
        .add_startup_system(initiate_project)
        .add_startup_system(start_websocket_server)
        // .add_system(ui_example_system)
        .add_system(build_message_log)
        .add_system(send_openai_prompt)
        .add_system(process_text)
        // .add_system(keyboard_input)
        .run();
}
