# Multicrank
### A server wrapper API for easier handling of market cranking.

## Building
* add musl toolchain
```
    rustup target add x86_64-unknown-linux-musl
```
* build musl target
```
    cargo build --bin multicrank --target x86_64-unknown-linux-musl --release
```

## Arguments
### Mandatory
* `--crank /path/to/bin/crank`
* - clob crank binary
* `--gas-payer /path/to/id.json`
* - private key of gas payer
* `--socket 8080` 
* - server socket
### Optional
* `--markets /path/to/markets.json`
* - a list of markets to pre-load
* `--persist /path/to/custom/persistance/folder`
* - a custom folder for state and log storage
* - default is `~/.multicrank`
### Example
```bash
./multicrank --crank /usr/bin/crank --gas-payer ./id.json \
 --socket 3030 --markets .dex/multicrank/markets_example.json
```


## API
### `POST /start_crank`
* starts a market crank
* payload example 
```json
{
   "marketInfo": {
    "address": "7dLVkUfBVfCGkFhSXDCq1ukM9usathSgS716t643iFGF",
    "deprecated": false,
    "name": "ETH/USDT",
    "programId": "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin",
    "baseMintAddress": "2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk",
    "quoteMintAddress": "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
    "baseTokenAccount": "EnHeePTZK2KWAZTiTXnZgYuGw6Tcf1o9kJLbzPayk9ZF",
    "quoteTokenAccount": "BqsJazb58s925AjEHgnttBb5yuS2nnHt9nDZcBj8eJoc",
    "baseSymbol": "ETH",
    "quoteSymbol": "USDT"
  },
   "crankDuration": 5
}
```
* payload definition (JSON)

| Field                 | Mandatory | Description               | Default |
| --------------------- | --------- | ------------------------- | ------- |
| `"address"`           | yes       | pubkey of market          | -       |
| `"deprecated"`        | no        | is market inactive        | false   |
| `"name"`              | no        | name of market            | null    |
| `"baseMintAddress"`   | no        | base mint pubkey          | null    |
| `"quoteMintAddress"`  | no        | quote mint pubkey         | null    |
| `"baseTokenAccount"`  | yes       | base token wallet pubkey  | -       |
| `"quoteTokenAccount"` | yes       | quote token wallet pubkey | -       |
| `"baseSymbol"`        | no        | base token symbol         | null    |
| `"quoteSymbol"`       | no        | quote token symbol        | null    |
| `"crankDuration"`     | no        | crank runtime in minutes  | null    |

### `GET /active_cranks`
* gets a list of all running cranks
* response definition (JSON, list of items)

| Field                 | Mandatory | Description               | Default |
| --------------------- | --------- | ------------------------- | ------- |
| `"address"`           | yes       | pubkey of market          | -       |
| `"deprecated"`        | no        | is market inactive        | false   |
| `"name"`              | no        | name of market            | null    |
| `"baseMintAddress"`   | no        | base mint pubkey          | null    |
| `"quoteMintAddress"`  | no        | quote mint pubkey         | null    |
| `"baseTokenAccount"`  | yes       | base token wallet pubkey  | -       |
| `"quoteTokenAccount"` | yes       | quote token wallet pubkey | -       |
| `"baseSymbol"`        | no        | base token symbol         | null    |
| `"quoteSymbol"`       | no        | quote token symbol        | null    |

### `GET /purge/{market_id}`
* stops a crank for `market_id` and returns it
* response definition (JSON, optional)

| Field                 | Mandatory | Description               | Default |
| --------------------- | --------- | ------------------------- | ------- |
| `"address"`           | yes       | pubkey of market          | -       |
| `"deprecated"`        | no        | is market inactive        | false   |
| `"name"`              | no        | name of market            | null    |
| `"baseMintAddress"`   | no        | base mint pubkey          | null    |
| `"quoteMintAddress"`  | no        | quote mint pubkey         | null    |
| `"baseTokenAccount"`  | yes       | base token wallet pubkey  | -       |
| `"quoteTokenAccount"` | yes       | quote token wallet pubkey | -       |
| `"baseSymbol"`        | no        | base token symbol         | null    |
| `"quoteSymbol"`       | no        | quote token symbol        | null    |

## TODO:
* Log streaming endpoint
* Some more documentation
* RPC polling for market data
* Automatic wallet creation for new mints
