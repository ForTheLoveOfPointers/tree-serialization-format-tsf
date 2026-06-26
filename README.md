# TSF — Tree Serialization Format

TSF is a compact, deterministic, line-based textual format for serializing ordered trees. Unlike markup languages or presentation formats, its sole purpose is to represent hierarchical structures for transmission, storage, and reconstruction.

## Niche

TSF fills the gap between JSON/XML and ad-hoc line protocols. It provides:

- **Determinism** — guaranteed canonical form for hashing, caching, diffs, and digital signatures (unlike JSON, YAML).
- **Streamability** — parse without buffering a whole document; decode messages as they arrive byte-by-byte (unlike XML, JSON).
- **Simplicity** — parseable in ~80 lines of C with no library, no generics, no dynamic allocation pressure (unlike XML, JSON, YAML).
- **Human readability** — grep-friendly, one line per node, no bracket noise (unlike XML, JSON, CBOR).

It is **not** a markup language, a presentation format, or a replacement for HTML. It is a wire/interchange format for structured tree data where computational cost matters.

## Design

- **O(n)** single-pass parsing, streaming-friendly
- No closing tags, no backtracking, no syntactic ambiguity
- Exactly one canonical textual representation per tree
- Compact and human-readable

## Examples

### General tree

```
0 html
1 head
2 title "My page"
1 body
2 h1 "Hello"
2 div class="container"
```

### Structured logging

```
0 request
1 method POST
1 path "/api/orders"
1 duration_ms 127
1 status 201
1 order_id 8901
1 error null
0 request
1 method GET
1 path "/api/users/42"
1 duration_ms 5
1 status 404
1 error "user not found"
```

Tail the log and filter by depth-0 fields — each entry is a self-contained tree on independent lines.

### IoT telemetry (constrained device)

```
0 telemetry
1 device "living-room-temp"
1 ts 1717200000
1 readings
2 temperature 22.3
2 humidity 48.1
2 battery 0.87
```

A sensor hub (e.g. ESP32) produces these messages over TCP. The parser reads the depth integer, counts depth changes to know when the message ends — no JSON library required, parser fits in firmware.

### IPC / RPC message

Request:

```
0 call
1 method "user.create"
1 id uuid-a5f2c8
2 params
3 name "Alice"
3 email "alice@example.org"
3 roles
4 "admin"
4 "billing"
```

Response:

```
0 result
1 id uuid-a5f2c8
2 data
3 user_id 42
3 created_at "2026-06-01T12:00:00Z"
```

Error:

```
0 error
1 id uuid-a5f2c8
1 code ErrUserExists
1 message "email already registered"
```

Each message is depth-0, frames can be length-prefixed or sent one-per-line over a socket. O(n) decoding with zero lookahead.

### Configuration

```
0 server
1 host "0.0.0.0"
1 port 8080
1 tls true
1 cert
2 path "/etc/certs/server.pem"
2 key_path "/etc/certs/server.key"
0 logging
1 level "info"
1 outputs
2 "stdout"
2 "file:///var/log/app.log"
```

No indentation ambiguity, no bracket balancing, trivially machine-generated and machine-parsed.

## Current Status

This is an early-stage reference implementation (v0.1.0).

| Component        | Status     |
|------------------|------------|
| AST types        | ✓ Complete |
| Serializer (`Display`) | ✓ Complete |
| Lexer            | ✓ Complete |
| Parser           | ✓ Complete |
| Error handling   | ✓ Complete |
| Validator        | ⏳ Planned  |

The parser reads TSF from `stdin` and outputs canonical TSF on `stdout`. Error messages are printed to `stderr`.

## License

TBD
