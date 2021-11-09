pub use crate::models::{ConsolidatedData, Remark};
pub use crate::resadd::ResourceConsolidated;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mint {
    pub collection: String,
    pub symbol: String,
    pub transferrable: Option<i64>,
    pub sn: String,
    pub metadata: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NftConsolidated {
    pub changes: Vec<String>,                 //TODO fix whatever changes is
    pub children: Vec<String>,                //TODO fix whatever children is
    pub resources: Vec<ResourceConsolidated>, //TODO fix whatever resource is
    pub block: i64,
    pub collection: String,
    pub symbol: String,
    pub transferrable: i64,
    pub sn: String,
    pub metadata: String,
    pub priority: Vec<String>,
    pub owner: String,
    pub rootowner: String,
    pub reactions: HashMap<String, String>, //TODO fix reactions
    pub forsale: String,
    pub burned: String,
    pub properties: HashMap<String, String>, //TODO what is properties?
    pub pending: bool,
    pub id: String,
}

pub fn handleMint(r: Remark, block: i64, caller: String, data: &mut ConsolidatedData) {
    let u = urlencoding::decode(&r.value).unwrap().into_owned();
    let dec: Result<Mint, serde_json::Error> = serde_json::from_str(&u);
    match dec {
        Ok(v) => {
            let id = format!("{}-{}-{}-{}", block, v.collection, v.symbol, v.sn);
            let n = NftConsolidated {
                changes: Vec::new(),   //TODO fix whatever changes is
                children: Vec::new(),  //TODO fix whatever children is
                resources: Vec::new(), //TODO fix whatever resource is
                block: block,
                collection: v.collection,
                symbol: v.symbol,
                transferrable: if v.transferrable.is_some() {
                    v.transferrable.unwrap()
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
            let d = data.nfts.entry(id).or_insert(n);
        }
        Err(e) => {
            println!("handleMint error: {:?}, r: {:?}", e, r);
        }
    }
}
