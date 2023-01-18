import asyncio
import json
import random
import socket
import struct
from typing import Tuple, Generator

HOST, PORT = "255.255.255.255", 9999
DISCOVERY_QUERY = {
    "system": {"get_sysinfo": None},
}
IV = 171


class DiscoveryProtocol(asyncio.DatagramProtocol):
    def __init__(self) -> None:
        super().__init__()
        self.__transport = None

    def connection_made(self, transport: asyncio.transports.DatagramTransport) -> None:
        self.__transport = transport
        sock = transport.get_extra_info("socket")
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
        try:
            sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        except OSError as ex:
            print("Unable to set SO_REUSEADDR: %s", ex)

        self.send_datagram(json.dumps(DISCOVERY_QUERY))

    def datagram_received(self, data: bytes, addr: Tuple[str, int]) -> None:
        host, _ = addr
        if host != "127.0.0.1":
            message = self.decrypt(data)
            print(f"IN FROM {host}: '{message}'")
        return super().datagram_received(data, addr)

    @staticmethod
    def xor_payload(plain: bytes) -> Generator[int, None, None]:
        key = IV
        for b in plain:
            key = key ^ b
            yield key

    @staticmethod
    def xor_encrypted_payload(secret: bytes) -> Generator[int, None, None]:
        key = IV
        for b in secret:
            plain = key ^ b
            key = b
            yield plain

    @staticmethod
    def encrypt(payload: str) -> bytes:
        plainbytes = payload.encode()
        return struct.pack(">I", len(plainbytes)) + bytes(
            DiscoveryProtocol.xor_payload(plainbytes)
        )

    @staticmethod
    def decrypt(payload: bytes) -> str:
        return bytes(DiscoveryProtocol.xor_encrypted_payload(payload)).decode()

    def send_datagram(self, payload: str) -> None:
        if self.__transport is not None:
            print(f"OUT: '{payload}'")
            encrypted_payload = self.encrypt(payload)
            self.__transport.sendto(encrypted_payload[4:], (HOST, PORT))


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
