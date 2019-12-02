use std::{
    cmp::Ordering::*,
    sync::{Arc, Barrier},
};

use serenity::{
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
};

use crate::business;

mod shutdown_sender;
pub(crate) use self::shutdown_sender::shutdown_msg as send_sd_msg;

// TODO: load dynamicly
const TRADES_ID: ChannelId = ChannelId(603_769_735_867_400_193);
const LEADERBOARD_ID: ChannelId = ChannelId(603_772_537_809_272_833);
const PRICES_ID: ChannelId = ChannelId(603_772_502_484_713_472);

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
    // Set a handler for the `message` event - so
    // that whenever a new message is received -
    // the closure (or function) passed will be
    // called.
    //
    // Event handlers are dispatched through a
    // threadpool, and so multiple events can be
    // dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        // Return if were not in trades chanel.
        if msg.author.bot || msg.channel_id != TRADES_ID {
            return;
        }

        let mc = msg.content.to_lowercase();

        // Buy responce
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
        // Sell reponce
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
        // Info responce
        } else if mc.starts_with("!me") {
            let response = {
                let data = ctx.data.read();
                let bm = data.get::<BuisnessManManager>().unwrap();
                let hack: String = bm.lock().me_responce(&msg);
                hack
            };

            if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                println!("Error sending message: {:?}", why);
            }
        // Override
        } else if mc.starts_with("!buy") || mc.starts_with("!sell") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Invalid request") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Conected to discord");

        // Send message to server anounsing our
        // arrival.
        if let Err(why) =
            TRADES_ID.say(&ctx.http, "@investor The market is now open.")
        {
            println!("Error sending message: {:?}", why);
        }

        // Update prices

        // Get variables
        let data = ctx.data.read();
        let bm = data.get::<BuisnessManManager>().unwrap();
        let bml = bm.lock();
        let mut prices = bml.prices.iter().collect::<Vec<(_, _)>>();

        // Sort prices
        prices.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Equal));

        // Buffer for uploading
        let mut content = String::with_capacity(2000);

        // Loop over prices
        for (name, price) in prices {
            // Add a stonk
            content.push_str(&format!(
                "**_{}_**: {:.2}\n",
                name.replace("-", " "),
                price
            ));
            // If were about to exceed the message limit (2048)
            if content.len() > 1900 {
                // Send what we have
                if let Err(why) = PRICES_ID.say(&ctx.http, &content) {
                    println!("Error sending message: {:?}", why);
                }
                //Start afresh
                content.clear();
            }
        }
        if content.is_empty() {
            // Send the rest
            if let Err(why) = PRICES_ID.say(&ctx.http, &content) {
                println!("Error sending message: {:?}", why);
            }
        }

        // Update the leaderboard
        let mut traders = bml.traders.iter().collect::<Vec<(_, _)>>();
        traders.sort_by(|a, b| {
            let (a_val, b_val) =
                (a.1.value(&bml.prices), b.1.value(&bml.prices));
            let (a_dol, b_dol) = (a.1.cash(), b.1.cash());

            a_val
                .partial_cmp(&b_val)
                .unwrap_or(Equal)
                .then(b_dol.partial_cmp(&a_dol).unwrap_or(Equal))
                .reverse()
        });

        let t2 = traders.iter().map(|x| (x.0, x.1.value(&bml.prices)));
        let t3 = t2
            .map(|y| {
                (
                    match ctx.http.get_user(*y.0.as_u64()) {
                        Ok(u) => u.name,
                        Err(_) => String::from("[unclear]"),
                    },
                    y.1,
                )
            })
            .collect::<Vec<_>>();

        content.clear();
        let mut i = 1;
        for (user, money) in t3 {
            content.push_str(&format!("{}. **_{}_**: {:.2}\n", i, user, money));
            if content.len() > 1900 {
                if let Err(why) = LEADERBOARD_ID.say(&ctx.http, &content) {
                    println!("Error sending message: {:?}", why);
                }
                content.clear()
            }
            i += 1;
        }
        if content.is_empty() {
            if let Err(why) = LEADERBOARD_ID.say(&ctx.http, &content) {
                println!("Error sending message: {:?}", why);
            }
        }

        // Tell main thread were done.
        data.get::<BarrierManager>().unwrap().wait();
    }
}
