pub use crate::models::{Change, ConsolidatedData, Invalid, Remark};
pub use crate::resadd::ResourceConsolidated;
// pub use crate::send::Change;
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
    pub properties: HashMap<String, String>, //TODO what is properties?
    pub pending: bool,
    pub id: String,
}

// Cannot mint NFT that already exists
//TODO can't mint an NFT for a non-existent collection

pub fn handle_mint(r: Remark, block: i64, caller: String, data: &mut ConsolidatedData) {
    let u = urlencoding::decode(&r.value).unwrap().into_owned();
    let dec: Result<Mint, serde_json::Error> = serde_json::from_str(&u);
    match dec {
        Ok(v) => {
            let id = format!("{}-{}-{}-{}", block, v.collection, v.symbol, v.sn);
            if data.nfts.contains_key(&id) {
                // NFT with this ID already exists.  Push invalid event.
                data.invalid.push(Invalid {
                    op_type: String::from("MINT"),
                    block: block,
                    caller: caller,
                    object_id: r.value,
                    message: String::from(format!("[MINT] NFT already exists: {}", id)),
                });
                return;
            }

            let n = NftConsolidated {
                changes: Vec::<Change>::new(), //TODO fix whatever changes is
                children: Vec::<ChildConsolidated>::new(), //TODO fix whatever children is
                resources: Vec::new(),         //TODO fix whatever resource is
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
                owner: caller.clone(),
                rootowner: caller,
                reactions: HashMap::new(), //TODO fix reactions
                forsale: String::from("0"),
                burned: String::new(),
                properties: HashMap::new(), //TODO what is properties?
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
                object_id: r.value,
                message: String::from(format!("[MINT] Missing values: {}", e)),
            });
        }
    }
}
