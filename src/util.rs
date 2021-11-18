use super::models;
use serde_json;

use std::io::prelude::*;
use std::process::{Command, Stdio};

pub fn parse_line(line: &str) -> Result<models::RawRemark, serde_json::Error> {
    let l = serde_json::from_str(&line);
    return l;
}

//subkey inspect --network kusama HNZata7iMYWmk5RvZRTiAsSDhV8366zq2YGb3tLH5Upf74F --output-type json | jq -r .ss58Address
//15oF4uVJwmo4TdGW7VfQxNLavjCXviqxT9S1MgbjMNHr6Sp5

pub fn subkey_inspect(address: String) -> Result<String, String> {
    let mut s = String::new();
    let process = match Command::new("jq")
        .arg("-r")
        .arg(".ss58Address")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => return Err(format!("couldn't spawn jq: {}", why)),
        Ok(process) => process,
    };

    let input = Command::new("subkey")
        .arg("inspect")
        .arg("--network")
        .arg("kusama")
        .arg(address.clone())
        .arg("--output-type")
        .arg("json")
        .output()
        .unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e);
        });

    match process.stdin.unwrap().write_all(&input.stdout) {
        Err(why) => return Err(format!("couldn't write to jq stdin: {}", why)),
        Ok(_) => println!("sent to jq"),
    }

    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => return Err(format!("couldn't read jq stdin: {}", why)),
        Ok(_) => println!("jq good"),
    }
    match s.strip_suffix("\n") {
        Some(v) => s = v.to_string(),
        None => return Err(format!("no subkey value found for {}", address)),
    };

    Ok(s)
}
