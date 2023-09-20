# bati
Bitcoin address total investment

With this cli you can know if a Bech32 Bitcoin address is profitable or not.

It goes through each transaction of the given address and checks the USD price when the transaction was confirmed. 


## Running
```sh
$ cargo run -- <BECH32_ADDRESS>
```

## Building

```sh
$ cargo build
```

## Known issues and limitations
- This cli only supports Bech32 addresses.
- If an address has more than 30 transactions, CoinGecko api will fail with a `TOO_MANY_REQUESTS` status code.

## Other
I am not a Rust developer myself, so this app could definitely use help 😅

<img width="1152" alt="Screenshot 2023-09-20 at 10 59 25" src="https://github.com/fernandoporazzi/bati/assets/2279686/dd41d704-3e37-4aa0-aae6-60967494fbba">
