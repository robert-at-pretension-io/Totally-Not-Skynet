import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(("liminalnook.com", 420))
s.sendall(b"reset")
s.close()
