use async_openai::types::CreateCompletionRequestArgs;
use bevy::prelude::*;
use bevy::utils::HashMap;
use either::Either;
use nom::bytes::complete::take_while;
use nom::character::complete::{multispace0, newline};
use nom::sequence::tuple;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, char, space0},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{delimited, preceded},
    IResult,
};

use bevy_tokio_tasks::TokioTasksRuntime;
use bollard::container::Config;
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::image::CreateImageOptions;
use clap::Parser;
use futures_lite::StreamExt;

use bollard::errors::Error;
use bollard::Docker;
use std::fs;
use std::path::Path;
use std::{fs::File, io::BufRead};

use async_openai::Client;

use serde::{Deserialize, Serialize};
// use serde_json::Result;



  #[derive(Serialize, Deserialize, Debug)]
  struct SystemContext {
    objects: Vec<String>,
    functions: Vec<String>,
  }

  #[derive(Resource)]
  struct CurrentIteration {
    current_iteration: usize,
  }

  #[derive(Serialize, Deserialize, Debug)]
  struct TeamLeadContextInput {
    objects: Vec<String>,
    functions: Vec<String>,
    current_function: String,
  }

  #[derive(Serialize, Deserialize, Debug)]
  struct TeamLeadContextOutput {
    objects: Vec<String>,
    functions: Vec<String>,
    current_function: String,
    description: String,
    test_cases: Vec<TestCase>,
  }

  #[derive(Serialize, Deserialize, Debug)]
  struct TestCase {
    input: String,
    output: String,
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

#[derive(Resource)]
struct Cmd {
    cmd: Vec<String>,
}

#[derive(Resource)]
struct OpenAIObjects {
    client: Option<Client>,
}

#[derive(Resource)]
struct ContainerInfo {
    id: Option<String>,
}

#[derive(Resource)]
struct Settings {
    input_mode: InputMode,
    stage: Stage,
    max_iterations: usize,
}

impl Settings {
    fn next_mode(&mut self) {
        match self.input_mode {
            InputMode::DockerCommand => self.input_mode = InputMode::OpenAI,
            InputMode::OpenAI => self.input_mode = InputMode::DockerCommand,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputMode {
    DockerCommand,
    OpenAI,
}

#[derive(Debug)]
struct ImplementationDetails {
    filename: String,
    language: String,
    command: String,
    code: String,
}

fn parse_implementation_details(input: &str) -> IResult<&str, ImplementationDetails> {
    let (input, filename) = delimited(
        tag("[filename]"),
        take_until("[/filename]"),
        tag("[/filename]"),
    )(input)?;
    let (input, language) = delimited(
        tag("[language]"),
        take_until("[/language]"),
        tag("[/language]"),
    )(input)?;
    let (input, command) = delimited(
        tag("[command]"),
        take_until("[/command]"),
        tag("[/command]"),
    )(input)?;
    let (input, code) = delimited(tag("[code]"), take_until("[/code]"), tag("[/code]"))(input)?;

    Ok((
        input,
        ImplementationDetails {
            filename: filename.to_owned(),
            language: language.to_owned(),
            command: command.to_owned(),
            code: code.to_owned(),
        },
    ))
}


#[derive(Debug)]
struct Object {
    name: String,
    var_type: Type,
}

#[derive(Debug)]
struct FunctionSignature {
    name: String,
    inputs: Vec<Variable>,
    return_type: Type,
}

#[derive(Debug)]
struct Variable {
    name: String,
    var_type: Type,
}

#[derive(Debug, Resource, Clone, Copy)]
enum Stage {
    Architecting,
    Ticketing,
    Developing,
}

#[derive(Debug)]
enum Type {
    Int,
    Float,
    Double,
    Bool,
    String,
    Other(String),
}

fn parse_object(input: &str) -> IResult<&str, Object> {

    let mut parser = tuple((get_encapsulated_text, char(':'), parse_type, char(';')));

    let (input, (name, _, var_type, _)) = parser(input)?;

    Ok((
        input,
        Object {
            name: name.to_owned(),
            var_type,
        },
    ))
}

fn parse_objects(input: &str) -> IResult<&str, Vec<Object>> {
    
    let result : IResult<&str, &str> = take_until("[objects]")(input);
    
    println!("\n\n{:?}", result);

    // let result2 : IResult<&str, &str> = take_until("[/objects]")(input);

    // println!("\n\n{:?}\n\n", result2);
    
    let (input, _) = result?;

    println!("input: {:?}\n\n", input);

    let result : IResult<&str, &str> = take_until("[/objects]")(input);


    let (the_rest, parse_these_objects) = result?;

    let parse_these_objects = newline::<&str,nom::error::Error<&str>>(parse_these_objects).unwrap().0;

    println!("parse_objects: {:?}\n\n", parse_these_objects);
    print!("the_rest: {:?}\n", the_rest);

    let result = preceded(tag("[objects]"), many0(parse_object))(parse_these_objects).unwrap().1;


    print!("result: {:?}\n", result);
    // let result = tuple((tag("[objects]"), many0(parse_object), tag("[/objects]")))(input);

    // let (input, objects) =
    //     delimited(tag("[objects]"), many0(parse_object), tag("[/objects]"))(input)?;

     Ok((the_rest, result))
}

fn get_encapsulated_text(input: &str) -> IResult<&str, &str> {
    let result : IResult<&str, &str> = take_until(":")(input);
    return result;
}

fn parse_function_signature(input: &str) -> IResult<&str, FunctionSignature> {
    let (input, name) = preceded(tag(" "), alpha1)(input)?;
    let (input, _) = delimited(space0, char('('), space0)(input)?;
    let (input, inputs) = separated_list0(tag(", "), parse_variable)(input)?;
    let (input, _) = delimited(space0, tag(")"), space0)(input)?;
    let (input, _) = preceded(tag(" -> "), space0)(input)?;
    let (input, return_type) = parse_type(input)?;

    Ok((
        input,
        FunctionSignature {
            name: name.to_owned(),
            inputs,
            return_type,
        },
    ))
}

fn parse_variable(input: &str) -> IResult<&str, Variable> {
    let (input, name) = preceded(space0, alpha1)(input)?;
    let (input, _) = preceded(space0, char(':'))(input)?;
    let (input, var_type) = parse_type(input)?;

    Ok((
        input,
        Variable {
            name: name.to_owned(),
            var_type,
        },
    ))
}

fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((
        map(tag("int"), |_| Type::Int),
        map(tag("float"), |_| Type::Float),
        map(tag("double"), |_| Type::Double),
        map(tag("bool"), |_| Type::Bool),
        map(tag("string"), |_| Type::String),
        map(alpha1, |s: &str| Type::Other(s.to_owned())),
    ))(input)
}

fn parse_functions(input: &str) -> IResult<&str, Vec<FunctionSignature>> {
    let (input, functions) = delimited(
        tag("[functions]"),
        many0(parse_function_signature),
        tag("[/functions]"),
    )(input)?;

    Ok((input, functions))
}

fn parse_architecture_data(input: &str) -> serde_json::Result<SystemContext> {
    serde_json::from_str(input).map_err(|e| e.into())
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

fn setup_openai_client(mut commands: Commands) {
    let args = Args::parse();
    let api_key = args.api_key.clone();
    let client = Client::new().with_api_key(api_key);

    commands.insert_resource(OpenAIObjects {
        client: Some(client),
    });
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

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut commands: Commands,
    mut settings: ResMut<Settings>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        settings.next_mode();
        println!("Input Mode: {:?}", settings.input_mode);
        string.clear();
        return;
    }

    for ev in char_evr.iter() {
        print!("'{}'", ev.char);
        string.push(ev.char);
    }

    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", *string);
        commands.insert_resource(Cmd {
            cmd: vec![string.clone()],
        });
        string.clear();
    }
}

fn send_command(
    project_object: Res<ProjectObjects>,
    container_info: Res<ContainerInfo>,
    runtime: ResMut<TokioTasksRuntime>,
    commands: Commands,
    settings: ResMut<Settings>,
    current_iteration: ResMut<CurrentIteration>,
    cmd: ResMut<Cmd>,
    openai: Res<OpenAIObjects>,
) {
    if cmd.cmd.len() > 0 {
        match settings.input_mode {
            InputMode::DockerCommand => {
                if container_info.id.is_some() {
                    send_docker_command(project_object, container_info, runtime, commands, cmd);
                }
            }
            InputMode::OpenAI => {
                send_openai_command(project_object, runtime, cmd, openai, settings, current_iteration);
            }
        }
    }

    fn send_docker_command(
        project_object: Res<ProjectObjects>,
        container_info: Res<ContainerInfo>,
        mut runtime: ResMut<TokioTasksRuntime>,
        commands: Commands,
        mut cmd: ResMut<Cmd>,
    ) {
        let local_cmd = cmd
            .cmd
            .pop()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

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
                        cmd: Some(local_cmd),
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

enum ParsingObjects {
    Architecture(SystemContext),
    MakeTicket(TeamLeadContextInput),
    CompletedTicket(TeamLeadContextOutput),
    Implementation(ImplementationDetails),
}

fn opt_take_until_comment(i: &str) -> nom::IResult<&str, Option<&str>> {
    nom::combinator::opt(nom::bytes::complete::take_until("//"))(i)
  }

  fn remove_comments(i: &str) -> nom::IResult<&str, Vec<Option<&str>>> {
    nom::multi::many0(opt_take_until_comment)(i)
  }

fn parse_text(text: &str, stage: &Stage) -> Result<ParsingObjects, String> {
    match stage {
        Stage::Architecting => match parse_architecture_data(&text) {
            Ok(architecture_data) => return Ok(ParsingObjects::Architecture(architecture_data)),
            Err(e) => return Err(e.to_string()),
        },
        Stage::Ticketing => todo!(),
        Stage::Developing => todo!(),
    }
}

fn send_openai_command(
    project_object: Res<ProjectObjects>,
    runtime: ResMut<TokioTasksRuntime>,
    mut cmd: ResMut<Cmd>,
    openai: Res<OpenAIObjects>,
    settings: ResMut<Settings>,
    mut current_iteration: ResMut<CurrentIteration>,
) {
    cmd
        .cmd
        .pop();

    current_iteration.current_iteration += 1;

    if current_iteration.current_iteration > settings.max_iterations {
        println!("Max iterations reached");
        return;
    }

    let client = openai.client.clone().unwrap();

    // here is where we determine the prompt based on the stage of development
    let mut prompt = String::new();

    println!(
        "project_object prompt keys: {:?}",
        project_object.prompts.keys()
    );

    let local_setting = settings.stage;

    match settings.stage {
        Stage::Architecting => {
            prompt = project_object
                .prompts
                .get("softwareArchitect")
                .unwrap()
                .clone()
                .to_string();
            prompt = prompt + "{ goal: \"" + &project_object.goal.clone() + "\"}";
        }
        Stage::Ticketing => {
            prompt = project_object
                .prompts
                .get("teamLead")
                .unwrap()
                .clone()
                .to_string();
            prompt = prompt + "[goal]" + &project_object.goal.clone() + "[/goal]";
        }
        Stage::Developing => {
            prompt = project_object
                .prompts
                .get("developers")
                .unwrap()
                .clone()
                .to_string();
            prompt = prompt + "[goal]" + &project_object.goal.clone() + "[/goal]";
        }
    }

    runtime.spawn_background_task(move |ctx| async move {
        let mut finish_reason = Some("".to_string());
        let mut local_string = prompt.clone();
        let mut local_response = String::new();

        while finish_reason != Some("stop".to_string()) {
            let full_string = local_string.clone() + &local_response;
            let request = CreateCompletionRequestArgs::default()
                .model("text-davinci-003")
                .prompt(&full_string)
                .max_tokens(200_u16)
                .build()
                .unwrap();

            let response = client
                .completions() // Get the API "group" (completions, images, etc.) from the client
                .create(request) // Make the API call in that "group"
                .await
                .unwrap();

            let resp = &response.choices.first().unwrap().text;

            println!("Completions: {:?}", resp);

            local_response = local_response + &resp.clone().to_string();
            finish_reason = response.choices.first().unwrap().finish_reason.clone();

            if finish_reason == Some("stop".to_string()) {
                println!("Finished Reason: {:?}", finish_reason);
                println!("Local response: {}", local_response);

                let parsed_text = parse_text(&local_response, &local_setting);

                match parsed_text {
                    Ok(parsed) => 
                        {
                            match parsed {
                                ParsingObjects::Architecture(system_context) => {
                                    println!("System Context: {:?}", system_context);
                                    // settings.stage = Stage::Ticketing;
                                    todo!()
                                    
                                },
                                ParsingObjects::MakeTicket(_) => todo!(),
                                ParsingObjects::CompletedTicket(_) => todo!(),
                                ParsingObjects::Implementation(_) => todo!(),
                            }
                        }
                    Err(e) => println!("Error: {:?}", e),
                }
            }
        }
    });
}

fn print_project_objects(goal: Res<ProjectObjects>) {
    println!("Project Goals: \n------------------\n");
    println!("{}", goal.goal);
    println!("\n------------------\n");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_tokio_tasks::TokioTasksPlugin::default())
        .insert_resource(Cmd { cmd: vec![] })
        .insert_resource(ContainerInfo { id: None })
        .insert_resource(Settings {
            input_mode: InputMode::DockerCommand,
            stage: Stage::Architecting,
            max_iterations: 10,
        })
        .insert_resource(CurrentIteration { current_iteration: 0 })
        .init_resource::<ProjectObjects>()
        .add_startup_system(prepare_docker_container)
        // .add_startup_system(setup) // will add this back in when I figure out how to load a font
        .add_startup_system(print_project_objects)
        .add_startup_system(setup_openai_client)
        // .add_fixed_timestep(Duration::from_secs(5), "label")
        // .add_fixed_timestep_system("label", 0, print_container_info)
        .add_system(text_input)
        .add_system(send_command)
        .run();
}
