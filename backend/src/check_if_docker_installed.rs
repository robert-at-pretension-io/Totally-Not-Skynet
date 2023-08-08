use std::process::Command;
use std::fs;
use std::io;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DockerCheckError {
    #[error("Not running Ubuntu, can't install Docker automatically.")]
    NotUbuntu,
    #[error("Command execution failed: {0}")] CommandFailed(String),
}

pub fn docker_check() -> Result<()> {
    // Check if Docker is installed
    let output = Command::new("docker").arg("--version").output()?;

    if output.status.success() {
        println!("Docker is installed: {:?}", output);
        return Ok(());
    } else if is_ubuntu() {
        println!("Docker is not installed. Also, you're running ubuntu! Installing...");
        install_docker()?;
        return Ok(());
    } else {
        return Err(DockerCheckError::NotUbuntu.into());
    }
}

fn install_docker() -> Result<()> {
    // Step 1: Update existing list of packages
    run_command("sudo", &["apt-get", "update"])?;

    // Step 2: Install prerequisites
    run_command(
        "sudo",
        &[
            "apt-get",
            "install",
            "-y",
            "apt-transport-https",
            "ca-certificates",
            "curl",
            "software-properties-common",
        ]
    )?;

    // Step 3: Add Docker's official GPG key
    run_shell_command(
        "curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -"
    )?;

    let output = Command::new("lsb_release").arg("-cs").output()?;
    let codename = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Step 4: Set up the Docker stable repository
    let repo_url =
        format!("deb [arch=amd64] https://download.docker.com/linux/ubuntu {} stable", codename);
    run_command("sudo", &["add-apt-repository", &repo_url])?;

    // Step 5: Update the apt package index (again)
    run_command("sudo", &["apt-get", "update"])?;

    // Step 6: Install Docker
    run_command("sudo", &["apt-get", "install", "-y", "docker-ce"])?;

    println!("Docker installed successfully");
    Ok(())
}

fn run_shell_command(command_str: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(command_str).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(DockerCheckError::CommandFailed(command_str.to_string()).into())
    }
}

fn run_command(command: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(command).args(args).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(DockerCheckError::CommandFailed(command.to_string()).into())
    }
}

fn is_ubuntu() -> bool {
    // Read the contents of the /etc/os-release file
    match fs::read_to_string("/etc/os-release") {
        Ok(contents) => contents.contains("NAME=\"Ubuntu\""),
        Err(_) => false,
    }
}
