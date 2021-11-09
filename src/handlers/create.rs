pub use crate::models::{ConsolidatedData, Remark};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Create {
    pub max: i64,
    pub issuer: String,
    pub symbol: String,
    pub id: String,
    pub metadata: String,
}

//TODO fix this isn't right just copied from Base
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateConsolidated {
    pub changes: Vec<String>, //TODO fix this not sure what it's a vec of
    pub block: i64,
    pub max: i64,
    pub issuer: String,
    pub symbol: String,
    pub id: String,
    pub metadata: String,
}

pub fn handleCreate(r: Remark, block: i64, caller: String, data: &mut ConsolidatedData) {
    let u = urlencoding::decode(&r.value).unwrap().into_owned();
    let dec: Result<Create, serde_json::Error> = serde_json::from_str(&u);
    match dec {
        Ok(v) => {
            let c = CreateConsolidated {
                changes: Vec::new(), //TODO fix this not sure what it's a vec of
                block: block,
                max: v.max,
                issuer: caller,
                symbol: v.symbol,
                id: v.id.clone(),
                metadata: v.metadata,
            };
            let d = data.collections.entry(v.id.clone()).or_insert(c);
            //TODO handle checking collection stuffs
            // data.collections.insert(v.id, c);
        }
        Err(e) => {
            println!("Error with handleCreate: {:?} ::: {:?}", e, r);
        }
    }
}
