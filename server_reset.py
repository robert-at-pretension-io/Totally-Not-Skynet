import os
import socket
import subprocess
from signal import SIGTERM

def kill_process(name):
    subprocess.run(['pkill', '-f', name])

def git_pull(path):
    subprocess.run(['git', '-C', path, 'pull'])

def run_npm(path):
    subprocess.Popen(['npm', 'run', 'dev'], cwd=path)

def run_cargo(path):
    subprocess.Popen(['cargo', 'run'], cwd=path)

def listen_on_port(port, message):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('localhost', port))
        s.listen()
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

while True:
    if listen_on_port(80, "reset"):
        kill_process('rust')
        kill_process('npm')
        git_pull('~/projects/totally_not_skynet')
        run_npm('~/projects/totally_not_skynet/frontend')
        run_cargo('~/projects/totally_not_skynet/backend')
