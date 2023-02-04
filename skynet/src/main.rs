use bevy::prelude::*;
use clap::Parser;

// used to open file
use std::{fs::File, io::BufRead};

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = true)]
    debug: bool,
    #[clap(short, long, default_value = "project_goals.txt")]
    project_goals_file: String,
}

#[derive(Resource)]
struct ProjectGoals{
    goal : String
}

impl FromWorld for ProjectGoals {
    fn from_world(_world: &mut World) -> Self {
        let args = Args::parse();
        let mut goal = String::new();
        match File::open(args.project_goals_file) 
        {
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
            goal: goal
        }
    }
}

fn main() {
    App::new()
        .init_resource::<ProjectGoals>()
        .add_startup_system(print_project_goals)
        .run();
}

fn print_project_goals(goal: Res<ProjectGoals>) {
    println!("Project Goals: \n------------------\n");
    println!("{}", goal.goal);
    println!("\n------------------\n");

}