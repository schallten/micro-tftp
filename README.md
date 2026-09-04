# micro-tftp

Trivial file sharing over UDP, written in Python.

Chunked transfer (1024 B chunks), unlimited file size.

Handshake: client sends the filename, server replies with a 1-byte ACK, then the file is sent in chunks with a header (seq_num, size, is_last) and per-chunk ACKs.

## Usage

```
cd micro-tftp
python server.py   # listens on 0.0.0.0:8080
python client.py   # prompts for a filename
```

Files are saved to the server's working directory.

## Notes & precautions

- **Run the client and server in different working directories.** The server saves the file under the same name it receives, so if it runs from the same directory the client reads from, its `open(..., "wb")` truncates the source file to 0 bytes mid-transfer. Keep the sender and receiver in separate folders (e.g. `~/sender` and `~/receiver`).
- **Only one server per machine.** The server binds fixed port `8080`; starting a second server fails with `OSError: Address already in use`.
- **The protocol is strictly lock-step** — send one chunk, wait for a 1-byte ACK, send the next. Throughput is round-trip-time bound per chunk and does not scale across a real network (see benchmark below).
- Files sent down the same wire are never verified against the source — the transfer assumes a reliable, honest peer. There is no checksumming (MD5 in the benchmark was done externally, not by the program).

## Benchmark (1 GiB file, same machine)

| Metric                 | Value         |
| ---------------------- | ------------- |
| File size              | 1.0 GiB (1,073,741,824 B) |
| Chunks                 | 1,048,576 (1024 B each) |
| Elapsed                | 40.06 s       |
| Throughput             | ~25.6 MiB/s (~26.8 MB/s) |
| Integrity (MD5 match)  | yes           |

Conditions:

- Client and server running on the same machine/container (loopback `127.0.0.1`), Python 3.12
- UDP, 7-byte header (seq_num, size, is_last) + 1024-byte payload per packet
- Strictly lock-step protocol: send one chunk, wait for a 1-byte ACK, then send the next. Throughput is round-trip-time bound per chunk, so it does not scale to a full LAN/WAN link.
