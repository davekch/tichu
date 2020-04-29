Tichu
=======

Run the server:
```bash
git clone https://github.com/davekch/tichu.git
cd tichu
cargo build
sudo $(which cargo) run
```

Run the client (requires python3)
```bash
pip install pygame==2.0.0.dev6  # older versions cause cpu-usage of 100%
python python/client.py
```
