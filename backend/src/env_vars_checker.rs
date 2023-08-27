extern crate colored;

use colored::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;

pub fn check_env_vars(file_location: &str) -> io::Result<()> {
    let path = Path::new(file_location);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let var_name = line?;
        match env::var(&var_name) {
            Ok(_) => println!("{} : {} is set.", "✅ Success!".green(), var_name),
            Err(env::VarError::NotPresent) => {
                println!("{} : {} is NOT set.", "❌ Failure!".red(), var_name);
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("{} is not set.", var_name),
                ));
            }
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        }
    }
    Ok(())
}
