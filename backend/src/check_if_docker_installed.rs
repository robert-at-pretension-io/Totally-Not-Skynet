use std::process::Command;
use std::fs;

pub fn docker_check() {
    // Check if Docker is installed
    let output = Command::new("docker").arg("--version").output();

    if output.is_ok() {
        let version_str = String::from_utf8_lossy(&output.stdout);
        println!("Docker is installed: {}", version_str.trim());
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
    // The following commands are specific to Ubuntu.
    // Modify for your specific OS or distribution.
    let apt_update = Command::new("sudo")
        .arg("apt-get")
        .arg("update")
        .status()
        .expect("Failed to update apt repository");

    if !apt_update.success() {
        panic!("Failed to update apt repository");
    }

    let install_docker = Command::new("sudo")
        .arg("apt-get")
        .arg("-y")
        .arg("install")
        .arg("docker-ce")
        .arg("docker-ce-cli")
        .arg("containerd.io")
        .status()
        .expect("Failed to install Docker");

    if install_docker.success() {
        println!("Docker installed successfully");
    } else {
        panic!("Failed to install Docker");
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
