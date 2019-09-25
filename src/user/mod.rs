use std::{
    fs::File,
    io::{stdin, stdout},
};

use std::io::prelude::*;

use reqwest;

use crate::{business::BuisnessMan, groger::parse_groger_post};

pub const DATA_FILE_NAME: &str = "mdat.json";

pub(crate) fn interactive_bm_generate() -> BuisnessMan {
    // match  get_dat_file() {
    //     Some(v) => bm_from_json_string(&v),
    //     Err(_) => bm_from_no_file(),
    // }

    let dat_file_res = get_dat_file();

    if let Ok(dat_file) = dat_file_res {
        let mut yn_to_file = String::new();
        print!("Use existing prices [(Y)/n]: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut yn_to_file).unwrap();
        if yn_to_file.to_lowercase().starts_with('n') {
            let mut new_prices_bm = bm_from_no_file();
            let mut old_ownership_bm = bm_from_json_string(&dat_file);
            new_prices_bm.write = false;
            old_ownership_bm.write = false;

            BuisnessMan {
                prices:  new_prices_bm.prices.clone(),
                traders: old_ownership_bm.traders.clone(),
                write:   true,
            }
        } else {
            bm_from_json_string(&dat_file)
        }
    } else {
        println!("Failed to open database file");

        bm_from_no_file()
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
        bm_from_no_file()
    }
}

fn calc_price(_rank: u16, weight: f32) -> f64 {
    f64::from(weight)
}
