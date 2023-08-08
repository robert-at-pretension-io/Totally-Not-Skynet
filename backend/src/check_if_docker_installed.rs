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
    let cmd_str = "curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -";
    run_shell_command(cmd_str);

    let output = Command::new("lsb_release")
        .arg("-cs")
        .output()
        .expect("Failed to execute lsb_release");
    let codename = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Step 4: Set up the Docker stable repository
    let repo_url =
        format!("deb [arch=amd64] https://download.docker.com/linux/ubuntu {} stable", codename);
    run_command("sudo", &["add-apt-repository", &repo_url]);

    // Step 5: Update the apt package index (again)
    run_command("sudo", &["apt-get", "update"]);

    // Step 6: Install Docker
    run_command("sudo", &["apt-get", "install", "-y", "docker-ce"]);

    println!("Docker installed successfully");
}

fn run_shell_command(command_str: &str) {
    let status = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .status()
        .expect(&format!("Failed to run shell command: {}", command_str));

    if !status.success() {
        panic!("Shell command failed: {}", command_str);
    }
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
