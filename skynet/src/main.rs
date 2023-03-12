use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};
use bollard::container::Config;
use bollard::errors::Error;
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::image::CreateImageOptions;
use bollard::Docker;
use clap::Parser;
use tokio::runtime;
use core::panic;
use std::vec;
use futures_lite::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serpapi_search_rust::serp_api_search::SerpApiSearch;
use std::sync::{mpsc, Arc, Mutex};
use std::{fs::File, io::BufRead};

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

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    action: String,
    description: String,
    prompt: String,
}
impl Role {
    fn new(file_contents: String) -> Option<Role> {
        // deserialize the file contents
        match serde_json::from_str(&file_contents) {
            Ok(role) => Some(role),
            Err(e) => {
                println!("Error parsing role file: {}", e);
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

#[derive(Resource)]
struct Settings {
    project_folder: String,
    max_iterations: usize,
    write_file: String
}

#[derive(PartialEq, Eq)]
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
    available_actions: Vec<String>,
    roles : Option<Vec<Role>>,
    current_role: Option<Role>,
    recording_in_progress: bool,
    container_id: Option<String>,
    current_iteration: usize,
    files: Option<Vec<String>>,
    log: Option<Vec<String>>,
}

enum TerminalInfo {
    Input(String),
    Output(String),
    Err(String),
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

async fn get_search_results(){
        // read secret api key from environment variable
    // To get the key simply copy/paste from https://serpapi.com/dashboard.
    let params = HashMap::<String, String>::new();

    let args = Args::parse();

    let api_key = args.api_key_serp;

    println!("let's search about coffee on google");
    let mut params : std::collections::HashMap<String, String> = std::collections::HashMap::<String, String>::new();
    params.insert("q".to_string(), "coffee".to_string());
    params.insert("location".to_string(), "Austin, TX, Texas, United States".to_string());

    // initialize the search engine
    let search = SerpApiSearch::google(params, api_key);

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

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{ FromSample, Sample, SizedSample, SupportedStreamConfig};
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

fn prepare_docker_container(
    runtime: ResMut<TokioTasksRuntime>,
    mut runtime_settings: ResMut<RuntimeSettings>,
    mut settings: ResMut<Settings>,
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
    mut runtime_settings: ResMut<RuntimeSettings>,
    mut runtime: ResMut<TokioTasksRuntime>,
) {
    let id = runtime_settings.container_id.clone().unwrap();

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

fn initiate_project(mut runtime_settings: ResMut<RuntimeSettings>, mut commands: Commands) {
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
    let roles = helper_functions::load_prompts(&"./src/roles");

    runtime_settings.goal = Some(goal.clone());
    // loop through the roles and add the possible actions to the possible action list
    for role in roles.iter() {
        let action = &role.action;
        runtime_settings.available_actions.push(action.clone());
    }
    
    runtime_settings.roles = Some(roles.clone());
    let log_entry = format!("Goal: {}", goal.clone());
    runtime_settings.log = Some(vec!(log_entry));
    println!("Project Goals: \n------------------\n");
    println!("{}", goal);
    println!("\n------------------\n");

    commands.spawn((
        Unprocessed,
    ));
}

fn build_prompt(
    settings: ResMut<Settings>,
    mut runtime_settings: ResMut<RuntimeSettings>,
    mut query: Query<(Entity,  &mut Unprocessed)>,
    mut commands: Commands,
) {
    let mut current_iteration = runtime_settings.current_iteration.clone();
    for (the_entity, _unprocessed) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unprocessed>(); // We only want to process the entity once

        print!(
            "\nCurrent iteration: {:?}\n",
             &current_iteration
        );

        current_iteration += 1;
        runtime_settings.current_iteration = current_iteration;

        if current_iteration > settings.max_iterations {
            println!("Max iterations reached");
            return;
        }

        // here is where we determine the prompt based on the stage of development
        let mut prompt = String::new();

        let roles = runtime_settings.roles.clone().unwrap();
        // check to see if the current role is equal to "choose_action"
        let current_role = runtime_settings.current_role.clone();

        
        // if this is the first time through the loop
        if current_role.is_none() || current_role.clone().unwrap().action == "choose_action" {

                // get the role where the action is "choose_action"
                let current_role = roles.iter().find(|&role| role.action == "choose_action").unwrap();

                runtime_settings.current_role = Some(current_role.clone());
                
                // The prompt consists of the log appended to the top of the prompt for the current role

                // make the action : description pairs
                for role in roles.iter() {
                    prompt = prompt + &format!("\n{} : {}", role.action, role.description);
                }

                prompt = prompt + &format!("Action to take:");
        }
        else {
            // append the runtime log into a large string where the entries are separated by a newline
            let log : Vec<String> = runtime_settings.log.clone().unwrap();
            let mut acc = String::new();
           
            let role = current_role.clone().unwrap();

            for entry in log {
                acc = format!("{}\n{}", acc, entry)
            }

            prompt = format!("\n[{}]:[{}]\n{}", role.action.clone(), role.description.clone(), role.prompt.clone())
        
        }
        let role = current_role.clone().unwrap();



        println!("sending to open ai: [{}] : [{}]", &role.action, &role.description);

        commands
            .entity(the_entity)
            .insert(Prompt { text: prompt })
            .insert(Unsent);
    }
}

fn send_openai_prompt(
    // openai: Res<OpenAIObjects>,
    runtime: ResMut<TokioTasksRuntime>,
    mut query: Query<(Entity,&mut Prompt, &mut Unsent)>,
    mut commands: Commands,
) {
    for (the_entity, mut prompt, _unsent) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unsent>(); // We only want to process the entity once

        let local_prompt = prompt.as_mut().text.clone();

        let args = Args::parse();
        let api_key = args.api_key_openai.clone();

        runtime.spawn_background_task(move |mut ctx| async move {
            let mut finish_reason = "".to_string();

            let mut local_response = String::new();
            while finish_reason != "stop" {
                let full_string = local_prompt.clone() + &local_response.clone();

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
            }

            let super_local = local_response.clone();

            ctx.run_on_main_thread(move |mut ctx| {
                ctx.world
                    .entity_mut(the_entity.clone())
                    .insert(Unparsed { text: super_local });
            })
            .await;
        });
    }
}

fn process_text(
    mut query: Query<(Entity, &mut Unparsed)>,
    mut commands: Commands,
    mut runtime_settings: ResMut<RuntimeSettings>,
    mut settings: ResMut<Settings>,
) {
    let write_file = settings.write_file.clone();
    for (the_entity, unparsed) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unparsed>(); // We only want to process the entity once

        runtime_settings.log.as_mut().unwrap().push(unparsed.text.clone());

        // before going back into the prompt creation loop, we need to determine if the initialization agent has given a valid response. That is, it must be one of the actions in the list of actions.
        let available_actions = runtime_settings.available_actions.clone();

        // if the last action was 'choose_action' then we need to loop through the available actions and see if the response is one of them

        let current_role = runtime_settings.current_role.clone().unwrap();
        if current_role.action == "choose_action" {
            let mut valid_response = false;
            for action in available_actions.iter() {
                if unparsed.text.contains(action) {
                    valid_response = true;
                    // get the role corresponding to the action
                    let roles = runtime_settings.roles.clone().unwrap();
                    let current_role = roles.iter().find(|&role| role.action.to_string() == action.to_string()).unwrap();
                    runtime_settings.current_role = Some(current_role.clone());
                }
            }

            if !valid_response {
                println!("Invalid response. Please try again.");
                panic!("Invalid response. Please try again.");
            }
        }
        else {
            let roles = runtime_settings.roles.clone().unwrap();
            let current_role = roles.iter().find(|&role| role.action == "choose_action").unwrap();

            runtime_settings.current_role = Some(current_role.clone());
        }

        commands
        .entity(the_entity)
        .insert(Unprocessed);

    }
}


fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    runtime: Res<TokioTasksRuntime>,
    mut runtime_settings: ResMut<RuntimeSettings>,
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
        .insert_resource(RuntimeSettings {
            goal: None,
            available_actions: vec![],
            current_role: None,
            log: None,
            container_id: None,
            files: None,
            recording_in_progress: false,
            roles: None,
            current_iteration: 1,
        })
        .insert_resource(Settings {
            max_iterations: 10,
            write_file: "output.json".to_string(),
            project_folder: "project".to_string(),
        })
        .add_startup_system(prepare_docker_container)
        .add_startup_system(initiate_project)
        .add_system(build_prompt)
        .add_system(send_openai_prompt)
        .add_system(process_text)
        .add_system(keyboard_input)
        .run();
}