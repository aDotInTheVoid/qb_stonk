use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json;


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
    let hs: HashMap<String, (i16, f32)> = groger::parse_groger_post(&text).unwrap();
    
    let js = serde_json::to_string_pretty(&hs).unwrap();
    println!("{}", &js);
    let hs2: HashMap<String, (i16, f32)> = serde_json::from_str(&js).unwrap();

    assert_eq!(hs, hs2);
}
