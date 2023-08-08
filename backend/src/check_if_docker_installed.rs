use std::process::Command;
use std::fs;

pub fn docker_check() {
    // Check if Docker is installed
    let output = Command::new("docker").arg("--version").output();

    if output.is_ok() {
        println!("Docker is installed: {:?}", output.unwrap());
    } else {
        if is_ubuntu() {
            println!("Docker is not installed. Also, you're running ubuntu! Installing...");
            install_docker()
        } else {
            panic!("Not running ubuntu, can't install docker automatically.")
        }
    }
}

fn install_docker() {
    // Step 1: Update existing list of packages
    run_command("sudo", &["apt-get", "update"]);

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
    );

    // Step 3: Add Docker's official GPG key
    run_command(
        "curl",
        &[
            "-fsSL",
            "https://download.docker.com/linux/ubuntu/gpg",
            "|",
            "sudo",
            "apt-key",
            "add",
            "-",
        ]
    );

    // Step 4: Set up the Docker stable repository
    run_command(
        "sudo",
        &[
            "add-apt-repository",
            "deb",
            "[arch=amd64]",
            "https://download.docker.com/linux/ubuntu",
            "$(lsb_release -cs)",
            "stable",
        ]
    );

    // Step 5: Update the apt package index (again)
    run_command("sudo", &["apt-get", "update"]);

    // Step 6: Install Docker
    run_command("sudo", &["apt-get", "install", "-y", "docker-ce"]);

    println!("Docker installed successfully");
}

fn run_command(command: &str, args: &[&str]) {
    let status = Command::new(command)
        .args(args)
        .status()
        .expect(&format!("Failed to run command: {}", command));

    if !status.success() {
        panic!("Command failed: {}", command);
    }
}

fn is_ubuntu() -> bool {
    // Read the contents of the /etc/os-release file
    let contents = match fs::read_to_string("/etc/os-release") {
        Ok(contents) => contents,
        Err(_) => {
            return false;
        }
    };

    // Check if the contents contain the "Ubuntu" identifier
    contents.contains("NAME=\"Ubuntu\"")
}
