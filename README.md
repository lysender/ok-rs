# ok-rs

HTTP server that responds with OK, sometimes it echos.

Note: This is for development purposes only.

## Installation

Build using cargo:

```bash
cargo build --release
```

You can copy the binary into your `$PATH` to run it anywhere or
you can just run it directly.

```bash
./ok-rs --port 4200
```

## Testing

You can visit the URL directly using the browser or use `curl` for testing purposes.

```bash
curl http://127.0.0.1:4100/foo/bar/baz
```
