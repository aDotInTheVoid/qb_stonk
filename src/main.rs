use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod business;
mod discord;
mod groger;
mod user;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Portfolio {
    shares: HashMap<String, u64>,
    dollars: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MarketSnapshot {
    traders: HashMap<String, Portfolio>,
    prices: HashMap<String, f64>,
}

fn main() {
    let text =
        reqwest::get("https://grogerranks.com/2019/06/11/2019-post-nationals-overall-rankings/")
            .unwrap()
            .text()
            .unwrap();
    let hs = groger::parse_groger_post(&text);
    dbg!(hs);
}
