use std::{
    env,
    io::{stdin, stdout, Write},
    sync::{Arc, Barrier},
    thread,
};

use serenity::prelude::*;

mod business;
mod discord;
mod groger;
mod user;

use discord::{send_sd_msg, BarrierManager, BuisnessManManager, Handler};

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected to find the environment variable DISCORD_TOKEN");
    let startbarier = Arc::new(Barrier::new(2));

    let raw_bm = user::interactive_bm_generate();
    let businessman = Arc::new(Mutex::new(raw_bm));

    // Create client
    let mut client = Client::new(&token, Handler).expect("Err creating client");
    // Arc<Mutex<_>> of the shard (conection)
    // manager to shutdown with later.
    let shardman = client.shard_manager.clone();

    {
        // do this in a auxilary scope to drop Lock
        // on data
        let mut data = client.data.write();
        // Add the shardmanager to the client data
        data.insert::<BarrierManager>(startbarier.clone());
        data.insert::<BuisnessManManager>(businessman.clone());
    }

    // Start the client in a new thread so main
    // thread is free to capture input.
    let handle = thread::spawn(move || {
        // For now, just 1 shard
        if let Err(why) = client.start() {
            println!("Client error: {:?}", why);
        }
    });

    // This will block the tread until user
    // confermation.
    startbarier.wait();
    stdout().flush().unwrap();
    print!("Press enter to shutdown bot: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut String::new()).unwrap();

    // Shutdown the conection and return lock on the
    // manager.
    shardman.lock().shutdown_all();
    drop(shardman);

    // Wait for client tread to finish
    handle.join().unwrap();

    // Send a message that the markets have
    // shutdown.
    send_sd_msg();
    println!("Sucessfuly shut down");
}
