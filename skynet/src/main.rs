use bevy::ecs::system::Spawn;
use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use bollard::container::{AttachContainerOptions, Config, RemoveContainerOptions};
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::image::CreateImageOptions;
use clap::Parser;

use futures_lite::StreamExt;

use bollard::errors::Error;
use bollard::Docker;
use iyes_loopless::prelude::*;
use std::time::Duration;
use std::{fs::File, io::BufRead};

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = true)]
    debug: bool,
    #[clap(short, long, default_value = "project_goals.txt")]
    project_goals_file: String,
    #[clap(short, long, default_value = "alpine")]
    image: String,
    #[clap(long, default_value = "cool_project_name")]
    project_name: String,
}

#[derive(Resource)]
struct ProjectObjects {
    goal: String,
}

#[derive(Resource)]
struct ContainerInfo {
    id: Option<String>
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
        Self {
            goal,
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

fn prepare_docker_container(mut commands: Commands, runtime: ResMut<TokioTasksRuntime>, mut project_objects: ResMut<ProjectObjects>) {
    let args = Args::parse();
    let image = args.image.clone();
    let project_name = args.project_name.clone();
    runtime.spawn_background_task(|mut ctx| async move {
        let docker = new_docker().unwrap();
        let image = "ubuntu:latest";

        // see if the container already exists with a certain name:
        // let opts = ContainerListOpts::builder().all(true).build();
        // let containers = docker.containers().list(&opts).await.unwrap();

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

        let alpine_config = Config {
            image: Some(image),
            tty: Some(true),
            attach_stdin: Some(true),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            open_stdin: Some(true),
            ..Default::default()
        };

    let id = docker
        .create_container::<&str, &str>(None, alpine_config.clone().into()).await.unwrap().id;

        ctx.run_on_main_thread(move |ctx| {
            ctx.world.insert_resource(ContainerInfo {
                id: Some(id.clone()),
            });
        }).await;

        
    });

}

#[derive(Resource)]
struct Cmd {
    cmd: Vec<String>,
}

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut commands: Commands,
) {
    for ev in char_evr.iter() {
        println!("Got char: '{}'", ev.char);
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

fn send_command_to_container(
    project_object: Res<ProjectObjects>,
    container_info: Res<ContainerInfo>, 
    runtime: ResMut<TokioTasksRuntime>,
    mut commands: Commands,
    mut cmd: ResMut<Cmd>,
) {
    if cmd.cmd.len() > 0 && container_info.id.is_some() {
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_tokio_tasks::TokioTasksPlugin::default())
        .insert_resource(Cmd { cmd: vec![] })
        .insert_resource(ContainerInfo { id: None })
        .init_resource::<ProjectObjects>()
        .add_startup_system(prepare_docker_container)
        .add_startup_system(print_project_objects)
        // .add_fixed_timestep(Duration::from_secs(5), "label")
        // .add_fixed_timestep_system("label", 0, print_container_info)
        .add_system(text_input)
        .add_system(send_command_to_container)
        .run();
}

fn print_project_objects(goal: Res<ProjectObjects>) {
    println!("Project Goals: \n------------------\n");
    println!("{}", goal.goal);
    println!("\n------------------\n");

}

