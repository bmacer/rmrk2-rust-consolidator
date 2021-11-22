use log::warn;

use std::collections::HashMap;
use std::fs;

extern crate clap;
use clap::{App, Arg, SubCommand};

mod handlers;
mod models;
mod util;

use handlers::{
    accept, base, burn, buy, changeissuer, create, emote, equip, equippable, list, lock, mint,
    resadd, send, setpriority, setproperty, themeadd,
};
use models::*;
use util::*;

pub fn load_data(s: String) -> Result<ConsolidatedData, serde_json::Error> {
    let d = serde_json::from_str(&s);
    d
}

fn main() {
    env_logger::init();

    let matches = App::new("RMRK2.0 Consolidator (written in Rust)")
        .version("1.0")
        .author("Brandon Macer <bobbysox322@gmail.com>")
        .about("Converts a raw RMRK2.0 dump into a consolidated JSON file")
        .arg(
            Arg::with_name("output")
                .short("a")
                .long("append")
                // .value_name("input")
                .help("the output file (will append if exists or write if not)")
                .takes_value(true), // .required(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let input = matches.value_of("INPUT").unwrap();
    let default_output = format!("consolidated-{}", input);
    let output = matches.value_of("output").unwrap_or(&default_output);

    warn!("Beginning parsing");
    let mut type_count = HashMap::new();
    let mut data: ConsolidatedData;

    let f = fs::read_to_string(input);

    if f.is_err() {
        let err: std::io::Error = f.unwrap_err();
        println!("Error reading input file: {:?}", err);
        return;
    }

    match fs::read_to_string(output) {
        Err(_) => {
            println!(
                "No output file found for {}.  Will parse entire contents of input file {}",
                output, input
            );
            data = ConsolidatedData {
                nfts: HashMap::new(),
                collections: HashMap::new(),
                bases: HashMap::new(),
                invalid: Vec::new(),
                last_block: 0,
            };
        }
        Ok(v) => {
            println!("Output file found for {}.", output);
            let d: Result<ConsolidatedData, serde_json::Error> = serde_json::from_str(&v);
            match d {
                Err(e) => {
                    println!("Error loading current output JSON.  Try with a non-existent JSON file: {:?}", e);
                    return;
                }
                Ok(w) => {
                    println!(
                        "Latest block in {} is {}.  Will parse {} for blocks >= {}",
                        output, w.last_block, input, w.last_block
                    );
                    data = w;
                }
            }
        }
    }

    // data = ConsolidatedData {
    //     nfts: HashMap::new(),
    //     collections: HashMap::new(),
    //     bases: HashMap::new(),
    //     invalid: Vec::new(),
    //     last_block: 0,
    // };

    let f = fs::read_to_string(input).unwrap();
    let split_by_newline = f.split("\n");

    for line in split_by_newline {
        if line == "[" || line == "]" || line == "," {
            continue;
        }
        let parsed = parse_line(line);
        match parsed {
            Ok(v) => {
                if v.block < data.last_block {
                    println!("found existing block");
                    continue;
                }
                data.last_block = v.block;
                'callblock: for call in v.calls {
                    let decoded = hex::decode(&call.value[2..]);
                    let s = String::from_utf8(decoded.clone().unwrap());
                    let sp = s.clone().unwrap();
                    let s = sp.split("::");
                    let x: Vec<&str> = s.collect();
                    if x.len() < 3 {
                        println!("not enough args: {:?}", x);
                        continue 'callblock;
                    }
                    let protocol = x[0].to_string();
                    let method = x[1].to_string();
                    let version = x[2].to_string();
                    let mut url_encoded_value = String::new();
                    if method == "ACCEPT" {
                        // rmrk :: ACCEPT :: 2.0.0 :: 5105000-0aff6865bed3a66b-DLEP-DL15-00000001 :: RES :: V1i6B
                        if x.len() != 6 {
                            println!("not enough args in ACCEPT");
                            continue 'callblock;
                        }
                        accept::handle_accept(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "BASE" {
                        // rmrk::BASE::{version}::{html_encoded_json}
                        if x.len() != 4 {
                            println!("not correct number of args for BASE");
                            continue 'callblock;
                        }
                        base::handle_base(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "BURN" {
                        // rmrk :: BURN :: 2.0.0 :: 5105000-0aff6865bed3a66b-VALHELLO-POTION_HEAL-00000001
                        if x.len() != 4 {
                            println!("not correct number of args for BURN");
                            continue 'callblock;
                        }
                        burn::handle_burn(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "BUY" {
                        // rmrk::BUY::2.0.0::5105000-0aff6865bed3a66b-VALHELLO-POTION_HEAL-00000001
                        // RMRK::BUY::2.0.0::6-ALICES_COLLECTION-ALICES_NFT-001::FoQJpPyadYccjavVdTWxpxU7rUEaYhfLCPwXgkfD6Zat9QP
                        if x.len() != 4 && x.len() != 5 {
                            println!("not correct number of args for BUY");
                            continue 'callblock;
                        }
                        buy::handle_buy(x, call.extras, v.block, call.caller.clone(), &mut data);
                    } else if method == "CHANGEISSUER" {
                        // rmrk::CHANGEISSUER::2.0.0::0aff6865bed3a66b-DLEP::HviHUSkM5SknXzYuPCSfst3CXK4Yg6SWeroP6TdTZBZJbVT
                        if x.len() != 5 {
                            println!("not correct number of args for CHANGEISSUER");
                            continue 'callblock;
                        }
                        changeissuer::handle_changeissuer(
                            x,
                            v.block,
                            call.caller.clone(),
                            &mut data,
                        );
                    } else if method == "CREATE" {
                        // rmrk::CREATE::2.0.0::%7B%22max%22%3A100%2C%22issuer%22%3A%22CpjsLDC1JFyrhm3ftC9Gs4QoyrkHKhZKtK7YqGTRFtTafgp%22%2C%22symbol%22%3A%22DLEP%22%2C%22id%22%3A%220aff6865bed3a66b-DLEP%22%2C%22metadata%22%3A%22ipfs%3A%2F%2Fipfs%2FQmVgs8P4awhZpFXhkkgnCwBp4AdKRj3F9K58mCZ6fxvn3j%22%7D
                        if x.len() != 4 {
                            println!("not correct number of args for CREATE");
                            continue 'callblock;
                        }
                        create::handle_create(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "EMOTE" {
                        // RMRK::EMOTE::2.0.0::RMRK1::5105000-0aff6865bed3a66b-DLEP-DL15-00000001::1F389
                        if x.len() != 6 {
                            println!("not correct number of args for EMOTE");
                            continue 'callblock;
                        }
                        emote::handle_emote(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "EQUIP" {
                        // rmrk::EQUIP::2.0.0::5105000-0aff6865bed3a66b-DLEP-ARMOR-00000001::

                        if x.len() < 5 {
                            println!("SEND error, not enough args: {:?}", x);
                            continue;
                        }
                        // resource = x[3].to_string();
                        // slot = x[4].to_string();
                        equip::handle_equip(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "EQUIPPABLE" {
                        if x.len() < 5 {
                            println!("EQUIPPABLE error, not enough args: {:?}", x);
                            continue;
                        }
                        equippable::handle_equippable(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "LIST" {
                        // rmrk::LIST::2.0.0::5105000-0aff6865bed3a66b-VALHELLO-POTION_HEAL-00000001::10000000000
                        if x.len() != 5 {
                            println!("not correct number of args for LIST");
                            continue 'callblock;
                        }
                        list::handle_list(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "LOCK" {
                        // rmrk::LOCK::2.0.0::0aff6865bed3a66b-DLEP
                        //TODO LOCK logic is not implemented
                        if x.len() != 4 {
                            println!("not correct number of args for LIST");
                            continue 'callblock;
                        }
                        lock::handle_lock(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "MINT" {
                        // rmrk::MINT::{version}::{html_encoded_json}::{recipient?}
                        if x.len() != 4 && x.len() != 5 {
                            println!("not correct number of args for MINT");
                            continue 'callblock;
                        }
                        mint::handle_mint(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "RESADD" {
                        // rmrk::RESADD::{version}::{id}::{html_encoded_json}
                        if x.len() != 5 {
                            println!("not correct number of args for RESADD");
                            continue 'callblock;
                        }
                        resadd::handle_resadd(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "SEND" {
                        // rmrk::SEND::{version}::{id}::{recipient}
                        if x.len() != 5 {
                            println!("not correct number of args for SEND");
                            continue 'callblock;
                        }
                        send::handle_send(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "SETPRIORITY" {
                        // rmrk::SETPRIORITY::2.0.0::{id}::{html_encoded_value}
                        if x.len() != 5 {
                            println!("not correct number of args for SETPRIORITY");
                            continue 'callblock;
                        }
                        setpriority::handle_setpriority(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "SETPROPERTY" {
                        // rmrk::SETPROPERTY::2.0.0::{id}::{html_encoded_name}::{html_encoded_value}
                        if x.len() != 6 {
                            println!("not correct number of args for SETPROPERTY");
                            continue 'callblock;
                        }
                        setproperty::handle_setproperty(x, v.block, call.caller.clone(), &mut data);
                    } else if method == "THEMEADD" {
                        // rmrk::THEMEADD::{version}::{base_id}::{name}::{html_encoded_json}
                        //TODO THEMEADD logic is not implemented
                        if x.len() != 6 {
                            println!("not correct number of args for THEMEADD");
                            continue 'callblock;
                        }
                        themeadd::handle_themeadd(x, v.block, call.caller.clone(), &mut data);
                    }

                    let count = type_count.entry(method.clone()).or_insert(0);
                    *count += 1;
                }
            }
            Err(e) => {
                println!("error! {}", e);
                println!("line: {:?}\n\n", line);
            }
        }
    }
    let d = serde_json::to_string(&data);
    match d {
        Ok(v) => {
            std::fs::write(output, v).expect("writing to json failed");
        }
        Err(e) => println!("unable to parse back to json: {:?}", e),
    }
    println!("Total counts: {:?}", type_count);
}
