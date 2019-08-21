use serenity::{
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
};

use crate::business;

// TODO: load dynamicly
const TRADES_ID: ChannelId = ChannelId(603769735867400193);

pub(crate) struct Handler;

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
        if let Err(why) = TRADES_ID.say(&ctx.http, "@everyone THE MARKET IS OPEN, GET YOUR STONKS")
        {
            println!("Error sending message: {:?}", why);
        }
    }
}
