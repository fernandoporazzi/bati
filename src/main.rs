#![allow(unused)]

use std::any::Any;
use clap::Parser;
use reqwest::Client;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;

/// Search for a bitcoin address and display the total investment in USD.
#[derive(Parser)]
struct Cli {
    // The bitcoin address to look for
    address: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    confirmed: bool,
    block_height: i32,
    block_hash: String,
    block_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Vout {
    scriptpubkey: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    scriptpubkey_address: String,
    value: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Vin {
    txid: String,
    vout: i32,
    prevout: Vout,
    scriptsig: String,
    scriptsig_asm: String,
    // witness: Vec<String>,
    is_coinbase: bool,
    sequence: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    txid: String,
    version: i32,
    locktime: i64,
    vin: Vec<Vin>,
    vout: Vec<Vout>,
    size: i32,
    weight: i32,
    fee: i32,
    status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentPrice {
    current_price: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketData {
    market_data: CurrentPrice,
}

#[derive(Serialize, Deserialize, Debug)]
struct ValuesAndBlockTimes {
    amount_of_sats: i64,
    block_time: String,
    price_in_usd: f64,
    usd_value_when_received: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let args = Cli::parse();

    println!("Calculating total investment for address {}", &args.address);

    let uri = format!("https://mempool.space/api/address/{}/txs", &args.address);
    let resp = reqwest::get(uri)
        .await?;

    match resp.status() {
        reqwest::StatusCode::OK => {}
        reqwest::StatusCode::NOT_FOUND => {
            panic!("Could not fetch bitcoin address. Please inform a valid address.");
        }
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        }
    }

    let data: Vec<Transaction> = resp
        .json::<Vec<Transaction>>()
        .await?;

    let mut values_and_block_times: Vec<ValuesAndBlockTimes> = vec![];
    let mut amount_of_transactions = 0;

    for transaction in data {

        for vout in transaction.vout {
            if vout.scriptpubkey_address == args.address {
                amount_of_transactions += 1;

                let naive_datetime = NaiveDateTime::from_timestamp_millis(transaction.status.block_time.to_owned() * 1000).unwrap();

                let price_uri = format!("https://api.coingecko.com/api/v3/coins/bitcoin/history?date={}&localization=false", naive_datetime.format("%d-%m-%Y"));

                let price_response = reqwest::get(price_uri)
                    .await?;

                match price_response.status() {
                    reqwest::StatusCode::OK => {}
                    reqwest::StatusCode::TOO_MANY_REQUESTS => {
                        panic!("Too many requests");
                    }
                    _ => {
                        panic!("Uh oh! Something unexpected happened while fetching price data.");
                    }
                }

                let price_data: MarketData = price_response.json::<MarketData>().await?;
                let price_in_usd = price_data.market_data.current_price["usd"].to_owned();

                let mut vabt: ValuesAndBlockTimes = ValuesAndBlockTimes {
                    amount_of_sats: vout.value,
                    price_in_usd,
                    block_time: naive_datetime.format("%d-%m-%Y").to_string(),
                    usd_value_when_received: price_in_usd.to_owned() * (vout.value.to_owned() as f64 / 100000000.0),
                };

                values_and_block_times.push(vabt);
            }
        }
    }

    let mut total_sats: i64 = 0;
    let mut total_values_when_received: f64 = 0.0;

    for vabt in values_and_block_times {
        total_sats += vabt.amount_of_sats;
        total_values_when_received += vabt.usd_value_when_received;
    }

    println!("Total amout of sats received: {}", total_sats);
    println!("Total amount invested USD: {:.2}", total_values_when_received);
    // println!("Transactions: {}", amount_of_transactions);

    println!();
    println!("HINT: if you want to know whether or not your address is profitable, check the balance on https://mempool.space/address/{}", &args.address);

    Ok(())
}