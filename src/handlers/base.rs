use crate::models::{ConsolidatedData, Remark};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Base {
    // pub id: String,
    pub symbol: String,
    // pub transferrable: i64,
    #[serde(rename = "type")]
    pub media_type: String,
    pub issuer: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    #[serde(rename = "type")]
    pub part_type: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equippable: Option<Vec<String>>,
    pub z: i32,
    // src: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseConsolidated {
    pub changes: Vec<String>, //TODO fix whatever changes is
    pub block: i64,
    pub symbol: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub id: String,
    pub issuer: String,
    pub parts: Vec<Part>,
}

pub fn handleBase(r: Remark, block: i64, caller: String, data: &mut ConsolidatedData) {
    let u = urlencoding::decode(&r.value).unwrap().into_owned();
    let dec: Result<Base, serde_json::Error> = serde_json::from_str(&u);
    match dec {
        Ok(v) => {
            let id = format!("base-{}-{}", block, v.symbol);
            let base = BaseConsolidated {
                block: block,
                id: id.clone(),
                changes: Vec::new(), //TODO fix this not sure what it's a vec of
                issuer: caller,
                symbol: v.symbol,
                media_type: v.media_type,
                parts: v.parts,
            };
            let d = data.bases.entry(id).or_insert(base);
            // //TODO handle checking collection stuffs
        }
        Err(e) => {
            println!("e: {:?}, handleBase remark: {:?}", e, u);
        }
    }
}
