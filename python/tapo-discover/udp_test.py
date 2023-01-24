import json
from Crypto.PublicKey import RSA
import socket

HOST, PORT = "255.255.255.255", 20002

rsa_key = RSA.generate(1024)

payload = {"params": {"rsa_key": rsa_key.export_key("PEM").decode("UTF-8")}}
payload = json.dumps(payload)

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_UDP)
sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)

print(f"OUT: '{payload}'")
sock.sendto(payload.encode(), (HOST, PORT))
