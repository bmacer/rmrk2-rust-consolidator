use log::warn;

use std::collections::HashMap;
use std::fs;

mod handlers;
mod models;
mod util;

use handlers::{accept, base, burn, buy, changeissuer, create, equip, list, mint, resadd, send};
use models::*;
use util::*;

fn main() {
    env_logger::init();

    warn!("Beginning parsing");
    let mut type_count = HashMap::new();
    let mut data = ConsolidatedData {
        nfts: HashMap::new(),
        collections: HashMap::new(),
        bases: HashMap::new(),
        invalid: Vec::new(),
        last_block: 0,
    };
    // let f = fs::read_to_string("chunky-unconsolidated.txt").expect("errror reading file");
    // let f = fs::read_to_string("download.txt").expect("errror reading file");
    // let f = fs::read_to_string("res.txt").expect("errror reading file");
    // let f = fs::read_to_string("one.txt").expect("errror reading file");
    let f = fs::read_to_string("z-before.json").expect("errror reading file");
    // let f = fs::read_to_string("just-two.txt").expect("errror reading file");
    let split_by_newline = f.split("\n");

    // let mut wr = fs::write("hello.txt", "hi");
    let mut wr = String::new();

    for line in split_by_newline {
        // if line.contains("8949591") {
        //     println!("line {:?}", line);
        //     std::process::exit(0);
        // };
        if line == "[" || line == "]" || line == "," {
            continue;
        }
        let parsed = parse_line(line);
        match parsed {
            Ok(v) => {
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
                    // let vv = x[2];
                    // if vv == "1.0.0" {
                    //     if ones % 100000 == 0 {
                    //         println!("ones: {:?}", ones);
                    //     }
                    //     ones += 1;
                    //     continue 'callblock;
                    // }
                    let protocol = x[0].to_string();
                    let method = x[1].to_string();
                    let version = x[2].to_string();
                    let mut resource_to_add_maybe = String::new();
                    let mut url_encoded_value = String::new();
                    let mut recipient = String::new();
                    let mut resource = String::new();
                    let mut slot = String::new();
                    if method == "RESADD" {
                        if x.len() < 5 {
                            println!("RESADD error, not enough args: {:?}", x);
                            continue;
                        }
                        println!("x: {:?}", x);
                        resource_to_add_maybe = x[3].to_string();
                        url_encoded_value = x[4].to_string();
                        if resource_to_add_maybe.contains("ait_voucher_04.svg") {
                            println!(
                                "brandon: {:?}\n\n tashia {:?}",
                                resource_to_add_maybe, url_encoded_value
                            );
                            std::process::exit(0);
                        }
                    } else if method == "SEND" {
                        if x.len() < 5 {
                            println!("SEND error, not enough args: {:?}", x);
                            continue;
                        }
                        resource_to_add_maybe = x[3].to_string();
                        recipient = x[4].to_string();
                    } else if method == "EQUIP" {
                        if x.len() < 5 {
                            println!("SEND error, not enough args: {:?}", x);
                            continue;
                        }
                        resource = x[3].to_string();
                        slot = x[4].to_string();
                    } else if method == "ACCEPT" {
                        // rmrk :: ACCEPT :: 2.0.0 :: 5105000-0aff6865bed3a66b-DLEP-DL15-00000001 :: RES :: V1i6B
                        if x.len() != 6 {
                            println!("not enough args in ACCEPT");
                            continue 'callblock;
                        }
                        accept::handle_accept(x, v.block, call.caller.clone(), &mut data);
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
                        println!("line: {:?}", line);
                        println!("x: {:?}", x);
                        if x.len() != 4 && x.len() != 5 {
                            println!("not correct number of args for BUY");
                            continue 'callblock;
                        }
                        buy::handle_buy(x, call.extras, v.block, call.caller.clone(), &mut data);
                    } else if method == "CHANGEISSUER" {
                        // rmrk::CHANGEISSUER::2.0.0::0aff6865bed3a66b-DLEP::HviHUSkM5SknXzYuPCSfst3CXK4Yg6SWeroP6TdTZBZJbVT
                        println!("line: {:?}", line);
                        println!("x: {:?}", x);
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
                    } else if method == "LIST" {
                        // rmrk::LIST::2.0.0::5105000-0aff6865bed3a66b-VALHELLO-POTION_HEAL-00000001::10000000000
                        if x.len() != 5 {
                            println!("not correct number of args for LIST");
                            continue 'callblock;
                        }
                        list::handle_list(x, v.block, call.caller.clone(), &mut data);
                    } else {
                        url_encoded_value = x[3].to_string();
                    }
                    let r = Remark {
                        protocol: protocol,
                        method: method,
                        version: version,
                        value: url_encoded_value,
                    };
                    let count = type_count.entry(r.method.clone()).or_insert(0);
                    *count += 1;
                    match r.method.as_str() {
                        "BASE" => base::handle_base(r, v.block, call.caller, &mut data),
                        "CREATE" => create::handle_create(r, v.block, call.caller, &mut data),
                        "MINT" => mint::handle_mint(r, v.block, call.caller, &mut data),
                        "RESADD" => {
                            resadd::handle_resadd(
                                resource_to_add_maybe,
                                r,
                                v.block,
                                call.caller,
                                &mut data,
                            );
                        }
                        "SEND" => send::handle_send(
                            resource_to_add_maybe,
                            recipient,
                            v.block,
                            call.caller,
                            &mut data,
                        ),
                        "EQUIP" => {
                            equip::handle_equip(resource, slot, v.block, call.caller, &mut data)
                        }
                        _ => {}
                    }
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
            std::fs::write("cons.json", v).expect("writing to json failed");
        }
        Err(e) => println!("unable to parse back to json: {:?}", e),
    }
    println!("Total counts: {:?}", type_count);
    // fs::write("just-two.txt", wr);
}
