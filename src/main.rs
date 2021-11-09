use std::collections::HashMap;
use std::fs;

mod handlers;
mod models;
mod util;

use handlers::{base, create, mint, resadd};
use models::*;
use util::*;

fn main() {
    let mut type_count = HashMap::new();
    let mut data = ConsolidatedData {
        nfts: HashMap::new(),
        collections: HashMap::new(),
        bases: HashMap::new(),
        invalid: Vec::new(), //TODO fix this not sure what it's a vec of
        lastBlock: 0,
    };
    let f = fs::read_to_string("chunky-unconsolidated.txt").expect("errror reading file");
    let split_by_newline = f.split("\n");
    for line in split_by_newline {
        if line == "[" || line == "]" || line == "," {
            continue;
        }
        let parsed = parse_line(line);
        match parsed {
            Ok(v) => {
                data.lastBlock = v.block;
                for call in v.calls {
                    let decoded = hex::decode(&call.value[2..]);
                    let s = String::from_utf8(decoded.clone().unwrap());
                    let sp = s.clone().unwrap();
                    let s = sp.split("::");
                    let x: Vec<&str> = s.collect();
                    let protocol = x[0].to_string();
                    let method = x[1].to_string();
                    let version = x[2].to_string();
                    let mut resource_to_add_maybe = String::new();
                    let mut url_encoded_value = String::new();
                    if method == "RESADD" {
                        resource_to_add_maybe = x[3].to_string();
                        url_encoded_value = x[4].to_string();
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
                        "BASE" => base::handleBase(r, v.block, call.caller, &mut data),
                        "CREATE" => create::handleCreate(r, v.block, call.caller, &mut data),
                        "MINT" => mint::handleMint(r, v.block, call.caller, &mut data),
                        "RESADD" => {
                            resadd::handle_resadd(
                                resource_to_add_maybe,
                                r,
                                v.block,
                                call.caller,
                                &mut data,
                            );
                            println!("decoded: {:?}", x);
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                println!("error! {}", e);
            }
        }
    }
    let d = serde_json::to_string(&data);
    match d {
        Ok(v) => {
            std::fs::write("chunky-consolidated.json", v).expect("writing to json failed");
        }
        Err(e) => println!("unable to parse back to json: {:?}", e),
    }
    println!("Total counts: {:?}", type_count);
}
