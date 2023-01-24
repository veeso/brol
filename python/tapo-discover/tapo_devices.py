import asyncio
import json
from Crypto.PublicKey import RSA
import socket
from typing import Tuple

HOST, PORT = "255.255.255.255", 20002
PREAMBLE = b"\x02\x00\x00\x01\x01\xe5\x11\x00\x0c\x75\x89\xf6\x68\xac\xac\x2b"
IV = 171


class DiscoveryProtocol(asyncio.DatagramProtocol):
    def __init__(self) -> None:
        super().__init__()
        self.__transport = None
        self.__rsa_key = RSA.generate(2048)

    def connection_made(self, transport: asyncio.transports.DatagramTransport) -> None:
        self.__transport = transport
        sock = transport.get_extra_info("socket")
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
        try:
            sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        except OSError as ex:
            print("Unable to set SO_REUSEADDR: %s", ex)

        payload = {
            "params": {
                "rsa_key": self.__rsa_key.publickey().export_key("PEM").decode("UTF-8")
            }
        }
        self.send_datagram(json.dumps(payload))

    def datagram_received(self, data: bytes, addr: Tuple[str, int]) -> None:
        host, _ = addr
        if host != "127.0.0.1":
            message = data[16:].decode("UTF-8")
            print(f"IN FROM {host}: '{message}'")
        return super().datagram_received(data, addr)

    def send_datagram(self, payload: str) -> None:
        packet = PREAMBLE + payload.encode()
        if self.__transport is not None:
            print(f"OUT: '{packet}'")
            self.__transport.sendto(packet, (HOST, PORT))


async def run():
    loop = asyncio.get_event_loop()
    (listener, _protocol) = await loop.create_datagram_endpoint(
        DiscoveryProtocol, local_addr=("0.0.0.0", 0)
    )
    await asyncio.sleep(30)
    listener.close()


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(run())
    loop.close()
