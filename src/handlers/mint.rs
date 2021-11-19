pub use crate::models::{Change, ConsolidatedData, Invalid, Remark};
pub use crate::resadd::ResourceConsolidated;
pub use crate::send::ChildConsolidated;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mint {
    pub collection: String,
    pub symbol: String,
    pub transferable: Option<i64>,
    pub sn: String,
    pub metadata: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NftConsolidated {
    pub changes: Vec<Change>,
    pub children: Vec<ChildConsolidated>,
    pub resources: Vec<ResourceConsolidated>,
    pub block: i64,
    pub collection: String,
    pub symbol: String,
    pub transferable: i64,
    pub sn: String,
    pub metadata: String,
    pub priority: Vec<String>,
    pub owner: String,
    pub rootowner: String,
    pub reactions: HashMap<String, Vec<String>>,
    pub forsale: String,
    pub burned: String,
    pub properties: HashMap<String, String>,
    pub pending: bool,
    pub id: String,
}

// Fail if NFT that already exists
// Fail if collection doesn't exist
// Mint (with optional recipient field)

pub fn handle_mint(raw_parts: Vec<&str>, block: i64, caller: String, data: &mut ConsolidatedData) {
    let recipient: String;
    let mint_json_decoded = raw_parts[3];
    if raw_parts.len() == 5 {
        recipient = raw_parts[4].to_string();
    } else {
        recipient = caller.clone();
    }
    let u = urlencoding::decode(&mint_json_decoded)
        .unwrap()
        .into_owned();
    let dec: Result<Mint, serde_json::Error> = serde_json::from_str(&u);
    match dec {
        Ok(v) => {
            let id = format!("{}-{}-{}-{}", block, v.collection, v.symbol, v.sn);

            // Fail if NFT that already exists
            if data.nfts.contains_key(&id) {
                data.invalid.push(Invalid {
                    op_type: String::from("MINT"),
                    block: block,
                    caller: caller,
                    object_id: mint_json_decoded.to_string(),
                    message: String::from(format!("[MINT] NFT already exists: {}", id)),
                });
                return;
            }

            // Fail if collection doesn't exist
            if !data.collections.contains_key(&v.collection) {
                data.invalid.push(Invalid {
                    op_type: String::from("MINT"),
                    block: block,
                    caller: caller.clone(),
                    object_id: v.collection.clone(),
                    message: String::from(format!(
                        "[MINT] Collection doesn't exist: {}",
                        v.collection
                    )),
                });
                return;
            }

            let n = NftConsolidated {
                changes: Vec::<Change>::new(),
                children: Vec::<ChildConsolidated>::new(),
                resources: Vec::new(),
                block: block,
                collection: v.collection,
                symbol: v.symbol,
                transferable: if v.transferable.is_some() {
                    v.transferable.unwrap()
                } else {
                    1
                },
                sn: v.sn,
                metadata: v.metadata,
                priority: Vec::new(),
                owner: recipient.clone(),
                rootowner: recipient,
                reactions: HashMap::new(),
                forsale: String::from("0"),
                burned: String::new(),
                properties: HashMap::new(),
                pending: false,
                id: id.clone(),
            };
            data.nfts.entry(id).or_insert(n);
        }
        Err(e) => {
            data.invalid.push(Invalid {
                op_type: String::from("MINT"),
                block: block,
                caller: caller,
                object_id: mint_json_decoded.to_string(),
                message: String::from(format!("[MINT] Missing values: {}", e)),
            });
        }
    }
}
