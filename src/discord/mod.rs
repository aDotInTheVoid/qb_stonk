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
pub(crate) struct BuisnessManManager;

impl TypeMapKey for BarrierManager {
    type Value = Arc<Barrier>;
}

impl TypeMapKey for BuisnessManManager {
    type Value = Arc<Mutex<business::BuisnessMan>>;
}

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.channel_id != TRADES_ID {
            return;
        }

        let mc = msg.content.to_lowercase();

        if mc.starts_with("!buy ") {
            let response = {
                // Drop locks
                let mut data = ctx.data.write();
                let bm = data.get_mut::<BuisnessManManager>().unwrap();
                let hack: String = bm.lock().buy_responce(&msg);
                hack
            };

            if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                println!("Error sending message: {:?}", why);
            }
        } else if mc.starts_with("!sell ") {
            let response = {
                let mut data = ctx.data.write();
                let bm = data.get_mut::<BuisnessManManager>().unwrap();
                let hack: String = bm.lock().sell_responce(&msg);
                hack
            };

            if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                println!("Error sending message: {:?}", why);
            }
        } else if mc.starts_with("!buy") || mc.starts_with("!sell") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Invalid request") {
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
