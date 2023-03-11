use std::{fs::{File, self, OpenOptions}, path::Path, io::BufWriter};

use bevy::utils::HashMap;
use serde::{Serialize, Deserialize};
use tar::Builder;



use std::io::Write;

use crate::Role;

pub fn contains_mostly_similar_strings(v1: &Vec<String>, v2: &Vec<String>) -> bool {
    // Make copies of both vectors so we can modify them safely.

    if v1.len() != v2.len() {
        // doesn't even contain the same number of strings
        return false;
    }

    let mut a = v1.clone();
    let mut b = v2.clone();

    // make all strings in a lowercase
    for i in 0..a.len() {
        a[i] = a[i].to_lowercase();
    }

    //same for b
    for i in 0..b.len() {
        b[i] = b[i].to_lowercase();
    }

    // Sort the vectors so we can compare them element-wise.
    a.sort();
    b.sort();

    println!("comparing {:?} to {:?}", a, b);

    let mut passes: Vec<bool> = Vec::new();
    // Check if there's any difference between the sorted vectors.
    for i in 0..a.len() {
        for j in 0..b.len() {
            let threshold: usize = (a[i].len() + b[j].len()) / 2 / 3; // 33% of the average length of the two strings
            if levenshtein_distance(&a[i], &b[j]) <= threshold {
                passes.push(true);
                // break out of the inner loop
                break;
            }
        }
    }

    if passes.len() == a.len() {
        return true;
    } else {
        return false;
    }
}

pub fn append_to_file<T: Serialize + Deserialize<'static>>(
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

pub fn load_prompts(directory_path : &str) -> Vec<Role> {
    
    let mut roles : Vec<Role> = Vec::new();
    let directory = Path::new(directory_path);

    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if file_path.is_file() {
            if let Some(file_name) = file_path.clone().file_name().and_then(|n| n.to_str()) {
                if let Some(file_stem) = Path::new(file_name).file_stem().and_then(|s| s.to_str()) {
                    let file_contents = fs::read_to_string(file_path).unwrap();
                    match Role::new( file_contents){
                        Some(role) => roles.push(role),
                        None => println!("Unable to parse: {:?}", file_stem),
                    }
                }
            }
        }
    }

    roles
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



pub fn levenshtein_distance(s: &str, t: &str) -> usize {
    let n = s.chars().count();
    let m = t.chars().count();

    if n == 0 || m == 0 {
        return n + m;
    }

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..=n {
        dp[i][0] = i;
    }
    for j in 0..=m {
        dp[0][j] = j;
    }

    for (i, sc) in s.chars().enumerate() {
        for (j, tc) in t.chars().enumerate() {
            let cost = if sc == tc { 0 } else { 1 };

            dp[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(dp[i][j + 1] + 1, dp[i + 1][j] + 1),
                dp[i][j] + cost,
            );
        }
    }

    dp[n][m]
}

pub fn get_function_names(v: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for s in v {
        let first_bracket_index = match s.find('(') {
            Some(index) => index,
            None => continue,
        };
        let function_name: String = s[..first_bracket_index].to_string();

        result.push(function_name.to_string());
    }
    result
}