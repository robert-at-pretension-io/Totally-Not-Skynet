use std::{fs::{File, self}, path::Path};

use tar::Builder;



use crate::Action;
use crate::Process;

pub fn load_actions(directory_path : &str) -> Vec<Action> {
    
    let mut actions : Vec<Action> = Vec::new();
    let directory = Path::new(directory_path);

    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if file_path.is_file() {
            if let Some(file_name) = file_path.clone().file_name().and_then(|n| n.to_str()) {
                if let Some(file_stem) = Path::new(file_name).file_stem().and_then(|s| s.to_str()) {
                    let file_contents = fs::read_to_string(file_path).unwrap();
                    match Action::new( file_contents){
                        Some(action) => actions.push(action),
                        None => println!("Unable to parse: {:?}", file_stem),
                    }
                }
            }
        }
    }

    actions
}

pub fn load_processes(directory_path : &str, available_actions: Vec<String>) -> Vec<Process> {
        
        let mut processes : Vec<Process> = Vec::new();
        let directory = Path::new(directory_path);
    
        for entry in fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            let file_path = entry.path();
    
            if file_path.is_file() {
                if let Some(file_name) = file_path.clone().file_name().and_then(|n| n.to_str()) {
                    if let Some(file_stem) = Path::new(file_name).file_stem().and_then(|s| s.to_str()) {
                        let file_contents = fs::read_to_string(file_path).unwrap();
                        match Process::new( file_contents){
                            Some(process) => {
                                // check to see that all of the actions are contained in the available actions
                                for action in process.steps.iter() {
                                    if !available_actions.contains(action) {
                                        println!("Unable to parse: {:?} because action {:?} is not available", file_stem, action)
                                    }
                                    else {
                                        processes.push(process.clone());
                                    }
                                }
                                
                            },
                            None => println!("Unable to parse: {:?}", file_stem),
                        }
                    }
                }
            }
        }
    
        processes
}

pub fn file_exists(file_name: &str) -> bool {
    let path = Path::new(file_name);
    path.is_file()
}

pub fn create_tarball(file_names: Vec<String>) -> std::io::Result<()> {
    // Create a new tar archive
let file = File::create("archive.tar.gz")?;
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

