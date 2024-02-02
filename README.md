# Minimal Rust HTTP server

To test timeouts.

```console
$ cargo run
   Compiling rust-sleep v0.1.0 (/tmp/rust-sleep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.81s
     Running `target/debug/rust-sleep`
2024-02-02T14:11:26.929532863+00:00 [warp::server][INFO] Server::run; addr=0.0.0.0:3030
2024-02-02T14:11:26.929618913+00:00 [warp::server][INFO] listening on http://0.0.0.0:3030
2024-02-02T14:11:29.452928776+00:00 [rust_sleep][INFO] Received a request to sleep
2024-02-02T14:11:34.453121450+00:00 [api_server][INFO] 127.0.0.1:51216 "GET /sleep HTTP/1.1" 200 "-" "curl/7.88.1" 5.000205534s
```

```console
$ curl -i http://127.0.0.1:3030/sleep
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 5
date: Fri, 02 Feb 2024 14:11:29 GMT
```
