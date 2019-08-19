use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Portfolio{
    shares: HashMap<String, u64>,
    dollars: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MarketSnapshot{
    traders: HashMap<String, Portfolio>,
    prices: HashMap<String, f64>,
}

fn get_prices_url_interactive(){
    do {
        let mut url = String::new();
    }    
}

fn main() {
   println!("Hello, world!");
}
