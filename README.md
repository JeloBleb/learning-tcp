# learning-tcp

A small file-transfer protocol I designed on top of raw TCP sockets, written in
Rust with no dependencies. A client can upload a file to the server or download
one back, using a binary wire format I defined by hand.

## Why I built it

I had been learning a lot of low-level CS in my Rust journey, and at the time I was interested in how network protocols worked, so I thought it would be interesting/beneficial for me to write some low-level code with TCP.

## The protocol

Every message starts with a 1-byte request type, then a length-prefixed
filename. Uploads also carry a length-prefixed payload. All integers are
big-endian.

**Upload** (type `0`):

| Field         | Size            |
|---------------|-----------------|
| type = `0`    | 1 byte          |
| filename len  | 1 byte          |
| filename      | *n* bytes UTF-8 |
| data len      | 4 bytes (u32)   |
| data          | *m* bytes       |

**Download** (type `1`): type, filename length, filename. The server answers
with an Upload-shaped message containing the file.

Encoding and decoding live in `src/lib.rs` (`TcpRequest`), shared by both
binaries, so the client and server can't drift apart on the format.

## Running it

```sh
cargo run --bin server   # listens on 127.0.0.1:7878, stores files in server_files/
cargo run --bin client   # uploads ./test, then downloads server-to-client into client_files/
```

## Limitations

- One request per connection; the server handles clients one at a time.
- Filenames max out at 255 bytes and files at 4 GiB (u8 and u32 length fields).
- No path sanitization: the server trusts the filename, so `../` could write
  outside `server_files/`. Fine for localhost learning, not for the internet.
- Errors mostly `unwrap()`; an unknown request type hits `todo!()`.

## What I learned

I learned how information is encoded into and decoded from raw bytes into readable data using a command/data format. It was surprisingly similar to SPI, which I had to deal with later in embedded programming. 
