import socket
from dataclasses import dataclass

CHUNK_SIZE: int = 1024
HEADER_SIZE: int = 7
MAX_PACKET: int = HEADER_SIZE + CHUNK_SIZE


@dataclass
class PacketHeader:
    seq_num: int
    size: int
    is_last: int


def pack_header(header: PacketHeader) -> bytes:
    return (
        header.seq_num.to_bytes(4, "little")
        + header.size.to_bytes(2, "little")
        + bytes([header.is_last])
    )


def send_file(sock: socket.socket, filename: str) -> None:
    seq_num: int = 0
    with open(filename, "rb") as file:
        sock.send(filename.encode())

        #ack_buf: bytes = sock.recv(1)
        # use later to check which chunks werent received and probably re-ask

        while True:
            chunk: bytes = file.read(CHUNK_SIZE)
            is_last: int = 1 if len(chunk) < CHUNK_SIZE else 0
            header: PacketHeader = PacketHeader(
                seq_num=seq_num, size=len(chunk), is_last=is_last
            )
            packet: bytes = pack_header(header) + chunk

            sock.send(packet)
            sock.recv(1)

            if seq_num % 10 == 0:
                print(f"  sent chunk {seq_num}")

            seq_num += 1
            if is_last == 1:
                break


def main() -> None:
    sock: socket.socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(("0.0.0.0", 0))
    sock.connect(("127.0.0.1", 8080))

    print("File sharing client")
    print("Enter filename to send (or 'exit' to quit ) : ")
    while True:
        filename: str = input("> ").strip()

        if filename == "exit":
            break

        if not filename:
            continue

        try:
            send_file(sock, filename)
            print("File sent successfully")
        except Exception as e:
            print(f"Error sending file: {e}")
        print("\nEnter next filename : ")


if __name__ == "__main__":
    main()
