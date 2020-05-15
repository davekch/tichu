Tichu
=======
Servers a single game of Tichu with four players.

Run the server:
```bash
git clone https://github.com/davekch/tichu.git
cd tichu
cargo build
sudo $(which cargo) run
```

Options:
```
tichuserver 0.1.0
davekch <dave-koch@web.de>
TCP server for a single game of Tichu

USAGE:
    tichu [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --ip_address <IP>    specify an IP address
    -p, --port <PORT>        specify a port
```

A client can be found at https://github.com/davekch/tichuclient
