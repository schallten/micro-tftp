# micro-tftp

Trivial file sharing over UDP, written in Rust and C.

Two independent implementations. They do **not** interoperate (different wire protocols).

| Implementation | Transfer method      | Max file size |
| -------------- | -------------------- | ------------- |
| Rust           | chunked (1024 B)     | unlimited     |
| C              | whole file, 1 packet | ~64 KB        |

Both use the same handshake: client sends the filename, server replies with a 1-byte ACK, then the file is sent.

## Rust

```
cd micro-tftp && cargo build --release
target/release/server   # listens on 0.0.0.0:8080
target/release/client   # prompts for a filename
```

Files are saved to the server's working directory.

## C

```
cd micro-tftp-c
gcc -O2 -o server server.c
gcc -O2 -o client client.c
./server                # listens on 0.0.0.0:8080
./client                # prompts for a filename
```

No safety checks, on purpose. Bigger than max size gets cut off.

Files are saved to the server's working directory.