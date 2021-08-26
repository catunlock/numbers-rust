import socket
from time import sleep
import sys

s = socket.create_connection(("localhost", 8000))
s.sendall(b"terminate")
s.close()