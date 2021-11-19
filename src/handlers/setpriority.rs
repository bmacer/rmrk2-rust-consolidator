pub use crate::mint::NftConsolidated;
pub use crate::models::{Change, ConsolidatedData, Invalid, Remark};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildConsolidated {
    pub id: String,
    pub pending: bool,
    pub equipped: String,
}

// Fail if NFT doesn't exist
// Fail if caller doesn't own the NFT
// Set priority (delete list and populate)

//TODO perhaps handle case where priorities are incomplete or incorrect?  at this point we accept whatever is sent, including mistakes

// rmrk::SETPRIORITY::2.0.0::{id}::{html_encoded_value}
pub fn handle_setpriority(
    raw_parts: Vec<&str>,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    let resource = raw_parts[3].to_string();
    let priorities_raw = raw_parts[4].to_string();

    // Fail if NFT doesn't exist
    if !data.nfts.contains_key(&resource) {
        data.invalid.push(Invalid {
            op_type: String::from("SETPRIORITY"),
            block: block,
            caller: caller,
            object_id: resource.clone(),
            message: String::from(format!("[SETPRIORITY] non-existent NFT {}", resource)),
        });
        return;
    }

    // Fail if caller isn't rootowner of NFT
    if data.nfts.get(&resource).unwrap().rootowner != caller {
        data.invalid.push(Invalid {
            op_type: String::from("SETPRIORITY"),
            block: block,
            caller: caller.clone(),
            object_id: resource.clone(),
            message: String::from(format!(
                "[SETPRIORITY] Caller {} does not own {}",
                caller, resource
            )),
        });
        return;
    };

    let decoded_string = urlencoding::decode(&priorities_raw).unwrap().into_owned();

    let priorities_as_vec: Vec<&str> = serde_json::from_str(&decoded_string).unwrap_or(Vec::new());

    if priorities_as_vec.len() == 0 {
        data.invalid.push(Invalid {
            op_type: String::from("SETPRIORITY"),
            block: block,
            caller: caller.clone(),
            object_id: decoded_string.clone(),
            message: String::from(format!(
                "[SETPRIORITY] improper format, should be [\"foo\",\"baz\"] {}",
                decoded_string
            )),
        });
        return;
    }

    let mut d = data.nfts.get_mut(&resource).unwrap();
    d.priority = Vec::new();
    for p in priorities_as_vec.iter() {
        d.priority.push(p.to_string())
    }
}
