pub use crate::models::{Change, ConsolidatedData, Invalid, Remark};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Create {
    pub max: i64,
    pub issuer: String,
    pub symbol: String,
    pub id: String,
    pub metadata: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateConsolidated {
    pub changes: Vec<Change>,
    pub block: i64,
    pub max: i64,
    pub issuer: String,
    pub symbol: String,
    pub id: String,
    pub metadata: String,
}

pub fn handle_create(r: Remark, block: i64, caller: String, data: &mut ConsolidatedData) {
    let u = urlencoding::decode(&r.value).unwrap().into_owned();
    let dec: Result<Create, serde_json::Error> = serde_json::from_str(&u);
    match dec {
        Ok(v) => {
            if data.collections.contains_key(&v.id.clone()) {
                // Collection with this ID already exists.  Push invalid event.
                data.invalid.push(Invalid {
                    op_type: String::from("CREATE"),
                    block: block,
                    caller: caller,
                    object_id: r.value,
                    message: String::from(format!("[CREATE] Collection already exists: {}", v.id)),
                });
                return;
            }
            let c = CreateConsolidated {
                changes: Vec::<Change>::new(),
                block: block,
                max: v.max,
                issuer: caller,
                symbol: v.symbol,
                id: v.id.clone(),
                metadata: v.metadata,
            };
            data.collections.entry(v.id.clone()).or_insert(c);
        }
        Err(e) => {
            data.invalid.push(Invalid {
                op_type: String::from("CREATE"),
                block: block,
                caller: caller,
                object_id: r.value,
                message: String::from(format!("[CREATE] Missing values: {}", e)),
            });
        }
    }
}
