# Description

Server for MPC protocol.

> This repo is inspired by [actix-sockets](https://github.com/antholeole/actix-sockets).

# Run

Server
```
cargo run
```

Client 1
```
git clone https://github.com/felicityin/mpc-node.git
cd mpc-node
cargo build

./target/debug/tss-cli keygen --server-url ws://127.0.0.1:8080 --room c05554ae-b4ee-4976-ac05-97aaf3c98a23 -i 0 -t 2 -n 2 output_1
```

Client 2
```
./target/debug/tss-cli keygen --server-url ws://127.0.0.1:8080 --room c05554ae-b4ee-4976-ac05-97aaf3c98a23 -i 1 -t 2 -n 2 output_2
```
