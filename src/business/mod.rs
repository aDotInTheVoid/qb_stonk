use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serenity::model::{channel::Message, id::UserId, user::User};

use crate::user::DATA_FILE_NAME;

const USER_DOLLARS_START: f64 = 1000.0;

// The property of 1 trader
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Portfolio {
    shares: HashMap<String, u64>,
    dollars: f64,
}

// A new trader has no stonks and USER_DOLLARS_START stonks
impl Portfolio {
    pub fn new() -> Self {
        Portfolio {
            shares: HashMap::new(),
            dollars: USER_DOLLARS_START,
        }
    }
}

// The state of the marker
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct BuisnessMan {
    // Maps a team name (as lowercase "-" speerated string) to price
    pub prices: HashMap<String, f64>,
    // Maps a UserID to a protfilis
    pub traders: HashMap<UserId, Portfolio>,
}

impl BuisnessMan {
    // New is empty
    pub fn new() -> Self {
        BuisnessMan {
            prices: HashMap::new(),
            traders: HashMap::new(),
        }
    }

    // Parse a json, and Posibly get self
    pub fn from_json(vals: &str) -> Option<Self> {
        match serde_json::from_str(vals) {
            Ok(v) => v,
            Err(_) => None,
        }
    }

    /// Execute the buy, and generate a responce
    pub fn buy_responce(&mut self, msg: &Message) -> String {
        // Parse The message
        let (num, name) = match Self::parse_buy_sell(&msg.content) {
            Err(v) => return v,
            Ok(v) => v,
        };

        // Try to get the price of the team the user requested.
        if let Some(price) = self.prices.get(&name.to_lowercase()) {
            // Calculate total prices of transaction
            let total_prices = (*price) * (num as f64);
            // Get the user, inserting a new trader if this is their first time
            let user_entry = self
                .traders
                .entry(msg.author.id)
                .or_insert(Portfolio::new());
            // Check if the user has enough money
            if user_entry.dollars < total_prices {
                return format!(
                    "This trade would cost {:.2}, but you only have {:.2}",
                    total_prices, user_entry.dollars
                );
            }

            // Remove the donnars
            user_entry.dollars -= total_prices;
            // Add the shares
            let num_shars: &mut u64 = user_entry.shares.entry(name.to_lowercase()).or_insert(0);
            *num_shars += num as u64;

            // Return the message
            format!(
                "<@{}>, You have baught {} stonks of {}, for a total of {:.2}. You now have {} of these stonks and {:.2} dollars",
                msg.author.id, num, name, total_prices, num_shars, user_entry.dollars
            )
        } else {
            // We don't have the stock
            format!(
                "It looks like you want to buy {}. However we don't know what this is. Check your spelling.",
                name
            )
        }
    }

    /// Execute the sell and send the responce
    pub fn sell_responce(&mut self, msg: &Message) -> String {
        let (num, name) = match Self::parse_buy_sell(&msg.content) {
            Err(v) => return v,
            Ok(v) => v,
        };

        format!(
            "It looks like you want to sell {} stonk(s) of {}. Unfortunalty this isnt suported",
            num, name
        )
    }

    pub fn me_responce(&self, msg: &Message) -> String {
        if let Some(user_entry) = self.traders.get(&msg.author.id) {
            format!(
                "<@{}>, you have {:.2} dollars and these shares:\n{}",
                msg.author.id, user_entry.dollars, display_shares(&user_entry.shares)
            )
        } else {
            format!(
                "<@{}>, You come from nothing. You're nothing. But not to me. \n (You have no stonks, but you do have your staring money of {} dollars)",
                msg.author.id, USER_DOLLARS_START
            )
        }
    }

    /// Parse a message for a name and a price.
    /// The name may not contain spaces.
    fn parse_buy_sell(mess: &String) -> Result<(u16, String), String> {
        // Normalise message
        let mess_lc = mess.to_lowercase();
        let mut parts = mess_lc.split(" ");

        // Skip over !buy / !sell
        parts.next().unwrap();

        // Get a number
        let num: u16 = match parts.next() {
            // No more text
            None => return Err("Incomplete request: Expected a quantity".to_owned()),
            // Found text
            Some(string) => match string.parse() {
                Ok(v) => v,                                                     // Can parse
                Err(_) => return Err(format!("Invalid number \"{}\"", string)), // Cant parse
            },
        };

        // Get a string name
        let name: &str = match parts.next() {
            None => return Err("Incomplete request: Expected a team".to_owned()), // No more text
            Some(v) => v,                                                         // More text
        };

        // Check their's nothing left
        match parts.next() {
            Some(v) => return Err(format!("Unexpected text: \"{}\"", v)),
            None => {}
        }

        Ok((num, name.to_owned()))
    }
}

impl Drop for BuisnessMan {
    // When the BuisnessMan drops, save the state
    fn drop(&mut self) {
        // Convert self to vec of json
        let json_self = serde_json::to_vec_pretty(&self)
            .expect("failed to sereialise state \n all trades may be lost");

        // Get paths
        let path = Path::new(DATA_FILE_NAME);
        let display = path.display();

        // Get file
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // Write the data to the file
        match file.write_all(&json_self) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}

fn display_shares(vals: &HashMap<String, u64>) -> String {
    let mut lines: Vec<String> = Vec::with_capacity(vals.len());
    for (i, j) in vals {
        lines.push(format!("{} of {}", i, j))
    }
    lines.join("\n")
}
