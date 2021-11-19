use crate::models::{ConsolidatedData, Invalid};
use serde_derive::{Deserialize, Serialize};

use crate::send::Change;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Base {
    // pub id: String,
    pub symbol: String,
    // pub transferable: i64,
    #[serde(rename = "type")]
    pub media_type: String,
    pub issuer: String,
    pub parts: Vec<Part>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Part {
    #[serde(rename = "type")]
    pub part_type: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equippable: Option<Vec<String>>,
    pub z: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseConsolidated {
    pub changes: Vec<Change>,
    pub block: i64,
    pub symbol: String,
    #[serde(rename = "type")]
    pub media_type: String,
    pub id: String,
    pub issuer: String,
    pub parts: Vec<Part>,
}

// rmrk::BASE::{version}::{html_encoded_json}
pub fn handle_base(raw_parts: Vec<&str>, block: i64, caller: String, data: &mut ConsolidatedData) {
    let r = raw_parts[3];
    let u = urlencoding::decode(&r).unwrap().into_owned();
    let dec: Result<Base, serde_json::Error> = serde_json::from_str(&u);
    match dec {
        Ok(v) => {
            let id = format!("base-{}-{}", block, v.symbol);

            // Fail if base already exists
            if data.bases.contains_key(&id) {
                data.invalid.push(Invalid {
                    op_type: String::from("BASE"),
                    block: block,
                    caller: caller,
                    object_id: r.to_string(),
                    message: String::from(format!("[BASE] Base already exists: {}", id)),
                });
                return;
            }

            let base = BaseConsolidated {
                block: block,
                id: id.clone(),
                changes: Vec::new(),
                issuer: caller,
                symbol: v.symbol,
                media_type: v.media_type,
                parts: v.parts,
            };
            data.bases.entry(id).or_insert(base);
        }
        Err(e) => {
            data.invalid.push(Invalid {
                op_type: String::from("BASE"),
                block: block,
                caller: caller,
                object_id: r.to_string(),
                message: String::from(format!("[BASE] Missing values: {}", e)),
            });
        }
    }
}
