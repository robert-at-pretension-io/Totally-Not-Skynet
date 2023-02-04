use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_tokio_tasks::TokioTasksRuntime;
use clap::Parser;
use docker_api::models::ContainerSummary;
use docker_api::opts::{ContainerCreateOpts, ContainerListOpts};
use docker_api::{Docker, Result};

use std::time::Duration;
use std::{fs::File, io::BufRead};
use iyes_loopless::prelude::*;


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
    docker: Result<Docker>,
    container_id: Option<String>,
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

        match new_docker() {
            Ok(docker) => Self {
                goal: goal,
                docker: Ok(docker),
                container_id: None,
            },
            Err(error) => {
                println!("Error: {}", error);
                println!("Make sure that Docker is installed and running.");
                Self {
                    goal: goal,
                    docker: Err(error),
                    container_id: None,
                }
            }
        }
    }
}

#[cfg(unix)]
fn new_docker() -> Result<Docker> {
    Ok(Docker::unix("/var/run/docker.sock"))
}

#[cfg(not(unix))]
fn new_docker() -> Result<Docker> {
    Docker::new("tcp://127.0.0.1:8080")
}

fn prepare_docker_container(mut commands: Commands, runtime: ResMut<TokioTasksRuntime>) {
    let args = Args::parse();
    let image = args.image.clone();
    let project_name = args.project_name.clone();
    runtime.spawn_background_task(|mut ctx| async move {
        let docker = new_docker().unwrap();

        // see if the container already exists with a certain name:
        let opts = ContainerListOpts::builder().all(true).build();
        let containers = docker.containers().list(&opts).await.unwrap();

        containers.iter().for_each(|container| {
            
            println!("\n\n---------------\n\nContainer: {:?}\n\n---------------\n\n", container);


        });



        let opts = ContainerCreateOpts::builder()
            .image(image)
            .name(project_name)
            .build();
        match docker.containers().create(&opts).await {
            Ok(container) => {
                println!("Docker Container Created!");
                println!("{:?}", container);
                ctx.run_on_main_thread(move |ctx| {

                let world: &mut World = ctx.world;
                world.get_resource_mut::<ProjectObjects>().unwrap().container_id = Some(container.id().to_string());

                }).await;
            }
            Err(error) => {
                println!("Error: {}", error);
                println!("Make sure that the 'image' has been installed. The default image is 'alpine'. Install it by running 'docker pull alpine' on the command line.")
            }
        }


    });
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)

        .add_plugin(bevy_tokio_tasks::TokioTasksPlugin::default())
        .init_resource::<ProjectObjects>()
        .add_startup_system(prepare_docker_container)
        .add_startup_system(print_project_objects)
        .add_fixed_timestep(Duration::from_secs(5), "label")
        .add_fixed_timestep_system("label", 0, print_container_info)
        .run();
}

fn print_project_objects(goal: Res<ProjectObjects>) {
    println!("Project Goals: \n------------------\n");
    println!("{}", goal.goal);
    println!("\n------------------\n");
    match goal.docker {
        Ok(_) => println!("Docker Found!"),
        Err(_) => println!("Docker Not Found!"),
    }
    match &goal.container_id {
        Some(id) => println!("Docker Container ID: {}", id),
        None => println!("Docker Container Not Created!"),
    }
}

fn print_container_info(project_object: Res<ProjectObjects>){
    match &project_object.container_id {
        Some(id) => {

            println!("Container Info: {:?}", &id);
        }
        None => println!("Docker Container Not Created!"),
    }
}
