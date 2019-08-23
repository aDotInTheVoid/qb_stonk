use std::sync::{Arc, Barrier};

use serenity::{
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
};

use crate::business;

mod shutdown_sender;
pub(crate) use self::shutdown_sender::shutdown_msg as send_sd_msg;

// TODO: load dynamicly
const TRADES_ID: ChannelId = ChannelId(603769735867400193);

pub(crate) struct Handler;
pub(crate) struct BarrierManager;

impl TypeMapKey for BarrierManager {
    type Value = Arc<Barrier>;
}

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.channel_id == TRADES_ID {
            if msg.content.to_lowercase().starts_with("!buy ") {
                let response = business::buy_responce(&msg);
                if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                    println!("Error sending message: {:?}", why);
                }
            }
        } else if msg.content.to_lowercase().starts_with("!sell ") {
            let response = business::sell_responce(&msg);
            if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Send message to server anounsing our arrival.
        if let Err(why) = TRADES_ID.say(
            &ctx.http,
            "@e_veryone THE MARKET IS OPEN, GET YOUR STONKS (not realy)",
        ) {
            println!("Error sending message: {:?}", why);
        }

        // Tell main thread were done.
        ctx.data.read().get::<BarrierManager>().unwrap().wait();
    }
}
