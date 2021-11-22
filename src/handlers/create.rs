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

// Fail if collection already exists
// Create collection

// rmrk::CREATE::2.0.0::%7B%22max%22%3A100%2C%22issuer%22%3A%22CpjsLDC1JFyrhm3ftC9Gs4QoyrkHKhZKtK7YqGTRFtTafgp%22%2C%22symbol%22%3A%22DLEP%22%2C%22id%22%3A%220aff6865bed3a66b-DLEP%22%2C%22metadata%22%3A%22ipfs%3A%2F%2Fipfs%2FQmVgs8P4awhZpFXhkkgnCwBp4AdKRj3F9K58mCZ6fxvn3j%22%7D

pub fn handle_create(
    raw_parts: Vec<&str>,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    let raw_create = raw_parts[3];
    let u: String;
    match urlencoding::decode(&raw_create) {
        Ok(v) => {
            u = v.into_owned();
        }
        Err(e) => {
            data.invalid.push(Invalid {
                op_type: String::from("CREATE"),
                block: block,
                caller: caller,
                object_id: raw_create.to_string(),
                message: String::from(format!("[CREATE] URL Decoding error: {} {}", raw_create, e)),
            });
            return;
        }
    }
    let create_decoded_into_json: Result<Create, serde_json::Error> = serde_json::from_str(&u);
    match create_decoded_into_json {
        Err(e) => {
            // Fail if URL decoding fails
            data.invalid.push(Invalid {
                op_type: String::from("CREATE"),
                block: block,
                caller: caller,
                object_id: u.clone(),
                message: String::from(format!("[CREATE] Missing values: {}", e)),
            });
            return;
        }
        Ok(v) => {
            if data.collections.contains_key(&v.id.clone()) {
                // Fail if collection already exists.
                data.invalid.push(Invalid {
                    op_type: String::from("CREATE"),
                    block: block,
                    caller: caller,
                    object_id: v.id.clone(),
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
    }
}
