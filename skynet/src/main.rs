use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_tokio_tasks::{TokioTasksRuntime, TaskContext};
use bollard::container::Config;
use bollard::errors::Error;
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::image::CreateImageOptions;
use bollard::Docker;
use clap::Parser;
use futures_lite::{StreamExt, Stream};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::{fs::File, io::BufRead};
use serpapi_search_rust::serp_api_search::SerpApiSearch;

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
        default_value ="a602132fb4dd2bad19d4df9532f26aa36d8bfadd8b08311f5fd96db7178b261c"
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
    write_file: String,
    project_phase: Phase
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
enum PlanningPhases {
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
    prompts: Option<HashMap<String, String>>,
    recording_in_progress: bool,
    container_id: Option<String>,
    current_iteration: usize,
    files: Option<Vec<String>>,
    project_phase: Phase,
    terminal_session: Option<Vec<TerminalInfo>>,
    project_progress: Option<Vec<PlanningPhases>>,
    all_functions: Vec<String>,
    implemented_functions: Vec<String>
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


// async fn get_search_results(){
//         // read secret api key from environment variable
//     // To get the key simply copy/paste from https://serpapi.com/dashboard.
//     let params = HashMap::<String, String>::new();
    
//     let args = Args::parse();

//     let api_key = args.api_key_serp;

//     println!("let's search about coffee on google");
//     let mut params : std::collections::HashMap<String, String> = std::collections::HashMap::<String, String>::new();
//     params.insert("q".to_string(), "coffee".to_string());
//     params.insert("location".to_string(), "Austin, TX, Texas, United States".to_string());

//     // initialize the search engine
//     let search = SerpApiSearch::google(params, api_key);

//     // search returns a JSON as serde_json::Value which can be accessed like a HashMap.
//     println!("waiting...");
//     let results = search.getJson(params).await.unwrap();
//     let organic_results = results["organic_results"].as_array().unwrap();
//     println!("results received");
//     println!("--- JSON ---");
//     println!(" - number of organic results: {}", organic_results.len());
//     println!(" - organic_results first result description: {}", results["organic_results"][0]["about_this_result"]["source"]["description"]);
//     let places = results["local_results"]["places"].as_array().unwrap();
//     println!("number of local_results: {}", places.len());
//     println!(" - local_results first address: {}", places[0]["address"]);

//     // search returns text
//     println!("--- HTML search ---");
//     let raw = search.html().await.unwrap();
//     print!(" - raw HTML size {} bytes\n", raw.len());
//     print!(" - async search completed with {}\n", results["search_parameters"]["engine"]);
//     print!("ok");
// }


use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use std::io::BufWriter;

#[derive(Parser, Debug)]
#[command(version, about = "CPAL record_wav example", long_about = None)]
struct Opt {
    /// The audio device to use
    #[arg(short, long, default_value_t = String::from("default"))]
    device: String,

    /// Use the JACK host
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    #[arg(short, long)]
    #[allow(dead_code)]
    jack: bool,
}

async fn record_audio(mut ctx : TaskContext) -> Result<(), anyhow::Error> {
    

    let mut response = ctx.run_on_main_thread(move |ctx| {
        ctx.world.get_resource::<RuntimeSettings>().unwrap().recording_in_progress.clone()
    }).await;

    println!("Recording in progress: {}", response);

    let (tx, rx) = oneshot::channel::<bool>();
    
    tokio::spawn(tokio_thread(rx));

    std::thread::sleep(std::time::Duration::from_secs(1));

    while response {
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        response = ctx.run_on_main_thread(move |ctx| {
            ctx.world.get_resource::<RuntimeSettings>().unwrap().recording_in_progress.clone()
        }).await;
        println!("Recording in progress: {}", response);
    }

 
    tx.send(true).unwrap();
    
    
    Ok(())
}



use tokio::sync::oneshot;

async fn tokio_thread(rx: oneshot::Receiver<bool>) {
    
        
        let opt = Opt::parse();

    // Conditionally compile with jack if the feature is specified.
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        ),
        feature = "jack"
    ))]
    // Manually check for flags. Can be passed through cargo with -- e.g.
    // cargo run --release --example beep --features jack -- --jack
    let host = if opt.jack {
        cpal::host_from_id(cpal::available_hosts()
            .into_iter()
            .find(|id| *id == cpal::HostId::Jack)
            .expect(
                "make sure --features jack is specified. only works on OSes where jack is available",
            )).expect("jack host unavailable")
    } else {
        cpal::default_host()
    };

    #[cfg(any(
        not(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd"
        )),
        not(feature = "jack")
    ))]
    let host = cpal::default_host();

    // Set up the input device and stream with the default input config.
    let device = if opt.device == "default" {
        host.default_input_device()
    } else {
        host.input_devices().unwrap()
            .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
    }
    .expect("failed to find input device");

    println!("Input device: {}", device.name().unwrap());

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    // The WAV file we're recording to.
    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/recorded.wav");
    let spec = wav_spec_from_config(config.clone());
    let writer = hound::WavWriter::create(PATH, spec).unwrap();
    let writer = Arc::new(Mutex::new(Some(writer)));

    // A flag to indicate that recording is in progress.
    println!("Begin recording...");

    // Run the input stream on a separate thread.
    let writer_2 = writer.clone();

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };
    
    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i8, i8>(data, &writer_2),
            err_fn,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
            err_fn,
            None,
        ),
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i32, i32>(data, &writer_2),
            err_fn,
            None,
        ),
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
            err_fn,
            None,
        )
    };
    

stream.unwrap().play();

let result = rx.await.unwrap();

if result {
// stream.unwrap().drop();
writer.lock().unwrap().take().unwrap().finalize().unwrap();
println!("Recording {} complete!", PATH);
}
    
}


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

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
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

fn prepare_docker_container(runtime: ResMut<TokioTasksRuntime>, mut runtime_settings: ResMut<RuntimeSettings>, mut settings: ResMut<Settings>) {
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
            ctx.world.get_resource_mut::<RuntimeSettings>().unwrap().container_id = Some(id);
        })
        .await;
    });
}

fn send_docker_command(
    mut runtime_settings: ResMut<RuntimeSettings>,
    mut runtime: ResMut<TokioTasksRuntime>
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
    let prompts = helper_functions::load_prompts(&"./src/prompts");

    runtime_settings.goal = Some(goal.clone());
    runtime_settings.prompts = Some(prompts.clone());

    println!("Project Goals: \n------------------\n");
    println!("{}", goal);
    println!("\n------------------\n");

    commands.spawn((
        PlanningPhases::SystemOrientation(InitialData { goal: goal.clone() }),
        Unprocessed,
    ));
}

fn build_prompt(
    settings: ResMut<Settings>,
    mut runtime_settings: ResMut<RuntimeSettings>,
    mut query: Query<(Entity, &mut PlanningPhases, &mut Unprocessed)>,
    mut commands: Commands,
) {
    let mut current_iteration = runtime_settings.current_iteration.clone();
    for (the_entity, mut planning_phase, _unprocessed) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unprocessed>(); // We only want to process the entity once

        print!(
            "Sending OpenAI Command: {:?}\nCurrent iteration: {:?}\n",
            &planning_phase, &current_iteration
        );

        current_iteration += 1;
        runtime_settings.current_iteration = current_iteration;

        if current_iteration > settings.max_iterations {
            println!("Max iterations reached");
            return;
        }

        // here is where we determine the prompt based on the stage of development
        let mut prompt = String::new();

        println!(
            "prompt keys: {:?}",
            runtime_settings.prompts.as_mut().unwrap().keys()
        );

        // let local_setting = settings.stage.clone();

        match planning_phase.as_mut() {
            PlanningPhases::SystemOrientation(initial_data) => {
                prompt = runtime_settings
                    .prompts
                    .as_mut()
                    .unwrap()
                    .get("softwareArchitect")
                    .unwrap()
                    .clone()
                    .to_string();
                prompt = prompt + &serde_json::to_string(&initial_data.clone()).unwrap();
            }
            PlanningPhases::Architecture(_) => {
                todo!()
            }
            PlanningPhases::MakeTicket(ticket_context) => {
                prompt = runtime_settings
                    .prompts
                    .as_mut()
                    .unwrap()
                    .get("teamLead")
                    .unwrap()
                    .clone()
                    .to_string();
                prompt = prompt + &serde_json::to_string(&ticket_context).unwrap();
            }
            PlanningPhases::CompletedTicket(ticket) => {
                prompt = runtime_settings
                    .prompts
                    .as_mut()
                    .unwrap()
                    .get("developers")
                    .unwrap()
                    .clone()
                    .to_string();
                prompt = prompt + &serde_json::to_string(&ticket).unwrap();
            }
            PlanningPhases::Implementation(_) => todo!(),
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
    mut query: Query<(Entity, &PlanningPhases, &mut Prompt, &mut Unsent)>,
    mut commands: Commands,
) {
    for (the_entity, _object, mut prompt, _unsent) in query.iter_mut() {
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

            // }
            // }
        });
    }
}

fn parse_text(
    mut query: Query<(Entity, &mut PlanningPhases, &mut Unparsed)>,
    mut commands: Commands,
    mut runtime_settings: ResMut<RuntimeSettings>,
    mut settings: ResMut<Settings>,
) {
    let write_file = settings.write_file.clone();
    for (the_entity, mut object, unparsed) in query.iter_mut() {
        commands.entity(the_entity).remove::<Unparsed>(); // We only want to process the entity once

        match object.as_mut() {
            PlanningPhases::SystemOrientation(_) => match parse_architecture_data(&unparsed.text) {
                Ok(architecture_data) => {
                    helper_functions::append_to_file(&write_file, &architecture_data.clone());
                    runtime_settings.all_functions =
                        helper_functions::get_function_names(&architecture_data.functions.clone());
                    println!("All functions: {:?}", runtime_settings.all_functions.clone());

                    if runtime_settings.project_progress.is_some() {
                        runtime_settings
                            .project_progress.as_mut()
                            .unwrap()
                            .push(PlanningPhases::Architecture(architecture_data.clone()));
                    }

                    for function in &architecture_data.functions {
                        let mut ticket = TeamLeadContextInput {
                            goal: runtime_settings.goal.as_ref().unwrap().clone(),
                            functions: architecture_data.functions.clone(),
                            currentFunction: function.clone(),
                            objects: architecture_data.objects.clone(),
                        };

                        commands.spawn((PlanningPhases::MakeTicket(ticket), Unprocessed));
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
            PlanningPhases::Architecture(_) => todo!(),
            PlanningPhases::MakeTicket(_) => match parse_ticket_data(&unparsed.text) {
                Ok(ticket_data) => {
                    helper_functions::append_to_file(&write_file, &ticket_data.clone());
                    if runtime_settings.project_progress.is_some() {
                        runtime_settings
                            .project_progress.as_mut()
                            .unwrap()
                            .push(PlanningPhases::CompletedTicket(ticket_data.clone()));
                    }
                    commands.spawn((PlanningPhases::CompletedTicket(ticket_data), Unprocessed));
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
            PlanningPhases::CompletedTicket(_) => {
                let json = parse_implementation_data(&unparsed.text);
                match json {
                    Ok(code) => {
                        helper_functions::append_to_file(&write_file, &code.clone());
                        runtime_settings
                            .implemented_functions
                            .push(code.currentFunction.clone());

                        let function_names: Vec<String> =
                            helper_functions::get_function_names(&runtime_settings.implemented_functions);

                        if helper_functions::contains_mostly_similar_strings(
                            &function_names,
                            &runtime_settings.all_functions,
                        ) {
                            println!("All functions implemented!");
                            runtime_settings.project_phase = Phase::Implementation;
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
            PlanningPhases::Implementation(_) => todo!(),
        };
    }
}

fn initiate_implementation(
    settings: ResMut<Settings>,
    mut runtime_settings: ResMut<RuntimeSettings>,
) {
    if runtime_settings.project_phase == Phase::Implementation {
        println!("Starting implementation");
        runtime_settings.files = Some(runtime_settings.implemented_functions.clone());
    }
}



fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    runtime: Res<TokioTasksRuntime>
    , mut runtime_settings: ResMut<RuntimeSettings>
) {
    if keys.just_pressed(KeyCode::Space) {
        // Space was just pressed
        if !runtime_settings.recording_in_progress{
            runtime.spawn_background_task(  |ctx| async  move {
                record_audio(ctx).await;
            });
            println!("Space was just pressed -- recording audio");
            runtime_settings.recording_in_progress = true;
        }
        else {
            println!("Space was just pressed -- stopping recording audio");
            runtime_settings.recording_in_progress = false;
        }
    }
    if keys.just_released(KeyCode::LControl) {
        // Left Ctrl was released
    }
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
        // .insert_resource(Cmd { cmd: vec![] })
        .insert_resource(RuntimeSettings {
            goal: None,
            container_id: None,
            files: None,
            recording_in_progress: false,
            project_progress: None,
            terminal_session: None,
            project_phase: Phase::Planning,
            prompts: None,
            current_iteration: 1,
            all_functions: vec![],
            implemented_functions: vec![],
            
        })
        .insert_resource(Settings {
            max_iterations: 10,
            write_file: "output.json".to_string(),
            project_phase: Phase::Planning,
            project_folder: "project".to_string(),
        })
        // .add_startup_system(prepare_docker_container)
        // .add_startup_system(initiate_project)
        // .add_system(build_prompt)
        // .add_system(send_openai_prompt)
        // .add_system(parse_text)
        // .add_system(initiate_implementation)
        .add_system(keyboard_input)
        .run();
}
