# Cranker CLI tool for the Serum fork DEX

## Building a binary

* add musl toolchain
```
    rustup target add x86_64-unknown-linux-musl
```
* build musl target from project root dir
```
    cargo build --bin crank --target x86_64-unknown-linux-musl --release
```

## Deploying

* copy the release binary to remote server
```
    scp ./target/x86_64-unknown-linux-musl/release/crank \
    user@remote-server.example:/path/to/bin/folder
```
* create a bash script on remote server
```
#!/bin/bash

touch /path/to/log/folder/log.txt

export CRANKER_DEX_PROGRAM_ID="dex-program-pubkey"
export CRANKER_PAYER_KEYPAIR_JSON="/path/to/fee/payer/keypair.json"
export CRANKER_MARKET_ID="market-pubkey"
export CRANKER_COIN_WALLET_ID="base-token-mint-pubkey"
export CRANKER_PC_WALLET_ID="trade-token-mint-pubkey"
export CRANKER_NUM_WORKERS=num_threads(example: 1)
export CRANKER_EVENTS_PER_WORKER=event_batch_size_per_worker(example: 10)
export CRANKER_LOG="/path/to/log/folder/log.txt"

/path/to/bin/folder/crank consume-events \
    --dex-program-id $CRANKER_DEX_PROGRAM_ID \
    --payer $CRANKER_PAYER_KEYPAIR_JSON \
    --market $CRANKER_MARKET_ID \
    --coin-wallet $CRANKER_COIN_WALLET_ID \
    --pc-wallet $CRANKER_PC_WALLET_ID \
    --num-workers $CRANKER_NUM_WORKERS \
    --events-per-worker $CRANKER_EVENTS_PER_WORKER \
    --log-directory $CRANKER_LOG
```

## Creating a daemon

* create systemd service file
```
nano example-crank.service
```
```
[Unit]
Description=Example cranker service
After=network.target
StartLimitIntervalSec=0
[Service]
Type=simple
Restart=always
RestartSec=1
User=user
ExecStart=/path/to/bash/script.sh

[Install]
WantedBy=multi-user.target
```
* activate service
```
sudo cp example-crank.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable example-crank.service
```
* start service
```
sudo systemctl start example-crank.service
```
* monitor logs
```
# (pretty useless right now, unless service crashes)
sudo journalctl -f -u example-crank.service
# binary native log
less /path/to/log/folder/log.txt
```

# TODO:
* ### multi market support
* ### fix market monitoring subcommand
* ### improve logging