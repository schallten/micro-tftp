import socket
from dataclasses import dataclass

CHUNK_SIZE: int = 1024
HEADER_SIZE: int = 7


@dataclass
class PacketHeader:
    seq_num: int
    size: int
    is_last: int


def unpack_header(data: bytes) -> PacketHeader:
    seq_num: int = int.from_bytes(data[0:4], "little")
    size: int = int.from_bytes(data[4:6], "little")
    is_last: int = data[6]
    return PacketHeader(seq_num=seq_num, size=size, is_last=is_last)


def receive_file(
    sock: socket.socket, client_addr: tuple[str, int], filename: str
) -> None:
    expected_seq: int = 0
    with open(filename, "wb") as output:
        while True:
            data, addr = sock.recvfrom(HEADER_SIZE + CHUNK_SIZE)
            if addr != client_addr:
                continue

            if len(data) < HEADER_SIZE:
                continue

            header: PacketHeader = unpack_header(data)

            if header.seq_num != expected_seq:
                continue

            if header.size > 0:
                output.write(data[HEADER_SIZE : HEADER_SIZE + header.size])

            sock.sendto(b"\x01", client_addr)

            if header.seq_num % 10 == 0:
                print(f"  Received chunk {header.seq_num}")

            if header.is_last == 1:
                break

            expected_seq += 1


def main() -> None:
    sock: socket.socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(("0.0.0.0", 8080))
    print("File Sharing Server listening on port 8080")

    while True:
        buf: bytes
        client_addr: tuple[str, int]
        buf, client_addr = sock.recvfrom(1024)
        filename: str = buf.decode(errors="replace")
        print(f"\nIncoming file: {filename} from {client_addr[0]}")

        sock.sendto(b"\x01", client_addr)

        try:
            receive_file(sock, client_addr, filename)
            print("File saved successfully")
        except Exception as e:
            print(f"Error receiving file: {e}")


if __name__ == "__main__":
    main()
