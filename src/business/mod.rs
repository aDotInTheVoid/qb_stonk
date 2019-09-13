use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serenity::model::{channel::Message, user::User};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Portfolio {
    shares: HashMap<String, u64>,
    dollars: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct MarketSnapshot {
    traders: HashMap<String, Portfolio>,
    prices: HashMap<String, f64>,
}

pub(crate) struct BuisnessMan {
    prices: HashMap<String, f64>,
    traders: HashMap<User, Portfolio>,
}

impl BuisnessMan {
    pub fn new() -> Self {
        BuisnessMan {
            prices: HashMap::new(),
            traders: HashMap::new(),
        }
    }

    /// Execute the buy, and generate a responce
    pub fn buy_responce(&mut self, msg: &Message) -> String {
        let (num, name) = match (Self::parse_buy_sell(&msg.content)) {
            Err(v) => return v,
            Ok(v) => v,
        };

        format!(
            "It looks like you want to buy {} stonk(s) of {}. Unfortunalty this isnt suported",
            num, name
        )
    }

    /// Execute the sell and send the responce
    pub fn sell_responce(&mut self, msg: &Message) -> String {
        let (num, name) = match (Self::parse_buy_sell(&msg.content)) {
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
        let mut parts = mess.split(" ");
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
