# bati
Bitcoin address total investment

With this cli you can know if a Bitcoin address is profitable or not.

It goes through each __Utxo__(_a certain amount of cryptocurrency that has been authorized by a sender and is available to be spent by a recipient_) of the given address and checks the USD price when the transaction occured. 


## Running
```sh
$ cargo run -- --address=<BECH32_ADDRESS> --delay=<OPTIONAL_DELAY_IN_SECONDS>
```

Due to a limitation on CoinGecko api, if an address has more than 30 utxos, the api will fail with `TOO_MANY_REQUESTS`. By adding a delay as env var when running the application, we can mitigate that. This deeply affects the UX, but it's the only option I could think of right now.


## Building

```sh
$ cargo build
```

## Known issues and limitations
- If an address has more than 30 transactions, CoinGecko api will fail with a `TOO_MANY_REQUESTS` status code.

## Other
I am not a Rust developer myself, so this app could definitely use help 😅

<img width="1152" alt="Screenshot 2023-09-20 at 10 59 25" src="https://github.com/fernandoporazzi/bati/assets/2279686/dd41d704-3e37-4aa0-aae6-60967494fbba">
