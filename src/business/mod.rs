use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serenity::model::{channel::Message, user::User};

use crate::user::DATA_FILE_NAME;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Portfolio {
    shares: HashMap<String, u64>,
    dollars: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct BuisnessMan {
    pub prices: HashMap<String, f64>,
    pub traders: HashMap<User, Portfolio>,
}

impl BuisnessMan {
    pub fn new() -> Self {
        BuisnessMan {
            prices: HashMap::new(),
            traders: HashMap::new(),
        }
    }

    pub fn from_json(vals: &str) -> Option<Self> {
        match serde_json::from_str(vals) {
            Ok(v) => v,
            Err(_) => None,
        }
    }

    /// Execute the buy, and generate a responce
    pub fn buy_responce(&mut self, msg: &Message) -> String {
        let (num, name) = match Self::parse_buy_sell(&msg.content) {
            Err(v) => return v,
            Ok(v) => v,
        };

        if let Some(price) = self.prices.get(name) {
            format!(
                "It looks like you want to buy {} stonk(s) of {}. \n They cost {} each for a total of {}\nUnfortunalty this isnt suported",
                num, name, price, (*price)*(num as f64)
            )
        } else {
            dbg!(&self.prices);
            format!(
                "It looks like you want to buy {}. However this isn't availibel",
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

    /// Parse a message for a name and a price.
    /// The name may not contain spaces.
    fn parse_buy_sell(mess: &String) -> Result<(u16, &str), String> {
        let mut parts = mess.make_ascii_lowercase().split(" ");
        // Skip over !buy / !sell
        parts.next().unwrap();

        let num: u16 = match parts.next() {
            None => return Err("Incomplete request: Expected a quantity".to_owned()),
            Some(string) => match string.parse() {
                Ok(v) => v,
                Err(_) => return Err(format!("Invalid number \"{}\"", string)),
            },
        };

        let name: &str = match parts.next() {
            None => return Err("Incomplete request: Expected a team".to_owned()),
            Some(v) => v,
        };

        match parts.next() {
            Some(v) => return Err(format!("Unexpected text: \"{}\"", v)),
            None => {}
        }

        Ok((num, name))
    }
}

impl Drop for BuisnessMan {
    fn drop(&mut self) {
        let json_self = serde_json::to_vec_pretty(&self)
            .expect("failed to sereialise state \n all trades may be lost");

        let path = Path::new(DATA_FILE_NAME);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(&json_self) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}
