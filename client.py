import socket
from time import sleep
import sys

s = socket.create_connection(("localhost", 8000))
buff = bytearray(100)

s.sendall(sys.argv[1].encode('utf-8'))
s.close()