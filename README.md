# Problem
When creating a post request with a `tokio` future in Rust within an `actix-web` executor, an error is thrown:

```
thread 'actix-rt:worker:1' panicked at 'Multiple executors at once: EnterError { reason: "attempted to run an executor while another executor is already running" }', src/libcore/result.rs:999:5
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
Panic in Arbiter thread, shutting down system.
```

# Run
```
cargo run
```

In other terminal window:
```
curl -X POST -H 'Content-Type: application/json' -d '{"test":1}' http://localhost:8080/push
```