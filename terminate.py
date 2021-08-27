import socket
from time import sleep
import sys

s = socket.create_connection(("localhost", 4000))
s.sendall(b"terminate")
s.close()

s = socket.create_connection(("localhost", 4000))
s.sendall(b"terminate")
s.close()