import socket
from time import sleep
import sys

while True:
    s = socket.create_connection(("localhost", 4000))
    buff = bytearray(100)

    s.sendall(sys.argv[1].encode('utf-8'))
    s.close()