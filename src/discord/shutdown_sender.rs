use std::env;
use std::sync::Arc;

use serenity::{client::bridge::gateway::ShardManager, model::gateway::Ready, prelude::*};

use super::TRADES_ID;

struct ShutdownHandler;

// Hax to allow Shardmanager to go in client data
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl EventHandler for ShutdownHandler {
    fn ready(&self, ctx: Context, _: Ready) {
        // First send message
        if let Err(why) = TRADES_ID.say(&ctx.http, "The market is now closed") {
            println!("Error sending message: {:?}", why);
        }

        // Get lock on data
        let data = ctx.data.read();

        // Try to optain shardmanager
        data.get::<ShardManagerContainer>()
            // It is almost certainly here
            .expect("We cant shutdown the api gracefully, things may break")
            .lock()
            // Send shutdown to discord.
            .shutdown_all();
    }
}

pub(crate) fn shutdown_msg() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, ShutdownHandler).expect("Err creating client");
    {
        // do this in a auxilary thread to drop Lock on data
        let mut data = client.data.write();
        // Add the shardmanager to the client data
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    // Only start once the shardmanager is added
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
