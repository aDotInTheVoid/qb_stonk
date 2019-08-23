use std::collections::HashMap;
use std::env;
use std::io::{stdin, stdout, Write};
use std::thread;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use serenity::prelude::*;

mod business;
mod discord;
mod groger;
mod user;

use discord::send_sd_msg;
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

    // Create client
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    // Arc<Mutex<_>> of the shard (conection) manager to shutdown with later.
    let shardman = client.shard_manager.clone();

    // Start the client in a new thread so main thread is free to capture input.
    let handle = thread::spawn(move || {
        // For now, just 1 shard
        if let Err(why) = client.start() {
            println!("Client error: {:?}", why);
        }
    });

    // This will block the tread until user confermation.
    //TODO: Dont sleep.
    thread::sleep(std::time::Duration::from_secs(2));
    stdout().flush().unwrap();
    print!("Press enter to shutdown bot: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut String::new()).unwrap();

    // Shutdown the conection and return lock on the manager.
    shardman.lock().shutdown_all();
    drop(shardman);

    // Wait for client tread to finish
    handle.join().unwrap();

    // Send a message that the markets have shutdown.
    send_sd_msg();
}
