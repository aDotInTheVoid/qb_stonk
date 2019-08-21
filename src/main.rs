use std::collections::HashMap;
use std::env;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use serenity::prelude::*;

mod business;
mod discord;
mod groger;
mod user;

use discord::Handler;

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
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
