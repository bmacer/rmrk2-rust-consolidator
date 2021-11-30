extern crate clap;
use clap::{App, Arg};
use handlers::{base, create, mint, resadd, send};
use log::debug;
use models::*;
use std::fs;

mod handlers;
mod models;
mod util;

fn main() {
    env_logger::init();
    debug!("Beginning parsing");

    let matches = App::new("RMRK2.0 Consolidator (written in Rust)")
        .version("1.0")
        .author("Brandon Macer <bobbysox322@gmail.com>")
        .about("Converts a raw RMRK2.0 dump into a consolidated JSON file")
        .arg(
            Arg::with_name("QUERY")
                .help("the query nft")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(false)
                .short("i")
                .long("input")
                .takes_value(true),
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap_or("chunky-perfect.json");
    let query = matches.value_of("QUERY").unwrap();

    let data: ConsolidatedData;

    match fs::read_to_string(input) {
        Err(_) => {
            println!("No input file found for {}", input);
            return;
        }
        Ok(v) => {
            // println!("input file found for {}.", input);
            let d: Result<ConsolidatedData, serde_json::Error> = serde_json::from_str(&v);
            match d {
                Err(e) => {
                    println!("Error loading current output JSON: {:?}", e);
                    return;
                }
                Ok(w) => {
                    // println!("Latest block in {} is {}", input, w.last_block);
                    data = w;
                }
            }
        }
    }

    let d = data.nfts.get(query);
    match d {
        None => {
            println!("no nft found for {:?}", query);
            return;
        }
        Some(v) => {
            for child in v.children.iter() {
                let id: Vec<&str> = child.id.split("-").collect();
                if id.len() >= 3 {
                    println!("{}", id[2]);
                }
            }
        }
    }
}
