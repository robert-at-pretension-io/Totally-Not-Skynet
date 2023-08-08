import os
import socket
import subprocess
import shutil
from signal import SIGTERM


def is_installed(command):
    return shutil.which(command) is not None


def ensure_dependencies():
    if not is_installed('cargo'):
        raise Exception("Cargo (Rust) is not installed. Please install it and try again.")
    
    if not is_installed('npm'):
        raise Exception("NPM is not installed. Please install it and try again.")


def kill_process(name):
    print(f"Attempting to kill all {name} processes...")
    subprocess.run(['pkill', '-f', name])
    print(f"All {name} processes killed.")


def git_pull(path):
    path = os.path.expanduser(path)
    print(f"Starting git stash in {path}...")
    subprocess.run(['git', '-C', path, 'stash', '--include-untracked'])

    print(f"Starting git pull in {path}...")
    subprocess.run(['git', '-C', path, 'pull'])
    print(f"Git pull in {path} completed.")


def run_npm(path):
    path = os.path.expanduser(path)

    # Running npm install first
    print(f"Starting npm install in {path}...")
    subprocess.run(['npm', 'install'], cwd=path)
    print(f"Npm install in {path} completed.")
    
    print(f"Starting npm run build in {path}...")
    subprocess.run(['npm', 'run', 'build'], cwd=path)
    print(f"Npm run build in {path} completed.")

    kill_process('rust')
    kill_process('skynet2')
    kill_process('npm')
    kill_process('node')

    print(f"Starting npm run start in {path}...")
    subprocess.Popen(['nohup', 'npm', 'run', 'start'], cwd=path,
                     stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.STDOUT, preexec_fn=os.setpgrp)
    print(f"Npm run start in {path} started.")


def run_cargo(path):
    path = os.path.expanduser(path)
    print(f"Starting cargo run in {path}...")
    subprocess.Popen(['nohup', 'cargo', 'run'], cwd=path,
                     stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.STDOUT, preexec_fn=os.setpgrp)
    print(f"Cargo run in {path} started.")


def listen_on_port(port, message):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        s.bind(('0.0.0.0', port))
        s.listen()
        print(f"Listening on port {port}...")
        conn, addr = s.accept()
        with conn:
            print('Connected by', addr)
            while True:
                data = conn.recv(1024)
                if not data:
                    break
                elif data.decode("utf-8") == message:
                    return True
    return False


# Ensure dependencies are installed
ensure_dependencies()

while True:
    if listen_on_port(420, "reset"):
        print("Received 'reset' command.")
        git_pull('~/projects/totally_not_skynet')
        run_npm('~/projects/totally_not_skynet/frontend')
        run_cargo('~/projects/totally_not_skynet/backend')
