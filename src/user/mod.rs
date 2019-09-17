use std::fs::File;
use std::io::{stdin,stdout};

use std::io::prelude::*;

use reqwest;

use crate::business::BuisnessMan;
use crate::groger::parse_groger_post;

pub const DATA_FILE_NAME: &str = "mdat.json";

pub(crate) fn interactive_bm_generate() -> BuisnessMan {
    // match  get_dat_file() {
    //     Some(v) => bm_from_json_string(&v),
    //     Err(_) => bm_from_no_file(),
    // }

    let x = get_dat_file();

    if x.is_ok() {
        return bm_from_json_string(&x.unwrap());
    } else {
        println!("Failed to open database file");

        return bm_from_no_file();
    }
}

/// Open a file and return it as a string
fn get_dat_file() -> std::io::Result<String> {
    let mut file = File::open(DATA_FILE_NAME)?;
    let mut ret = String::new();
    file.read_to_string(&mut ret)?;
    Ok(ret)
}

fn bm_from_json_string(json: &str) -> BuisnessMan {
    if let Ok(bm) = serde_json::from_str::<BuisnessMan>(json) {
        println!("Sucessfuly read file");
        bm
    } else {
        println!("Failed to read file");
        bm_from_no_file()
    }
}

fn bm_from_no_file() -> BuisnessMan {
    let mut groger_url = String::new();
    print!("Enter a groger url to get prices from: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut groger_url).unwrap();
    let grog_txt = reqwest::get(&groger_url).unwrap().text().unwrap();
    let _grog_vals = parse_groger_post(&grog_txt);
    if let Some(vals) = _grog_vals {
        let mut ret_bm = BuisnessMan::new();
        for (name, (rank, weight)) in vals.iter() {
            ret_bm
                .prices
                .insert(name.to_string(), calc_price(*rank, *weight));
        }
        ret_bm
    } else {
        print!("Failed to parse groger post");
        return bm_from_no_file();
    }
}

fn calc_price(rank: u16, weight: f32) -> f64 {
    weight as f64
}
