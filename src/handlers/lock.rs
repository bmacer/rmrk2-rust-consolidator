pub use crate::models::{Change, ConsolidatedData, Invalid, Remark};

// Fail if collection doesn't exist
// Fail if caller isn't issuer of collection

//TODO create and implement lock functionality ("locked" field in collection object?)

// rmrk::LOCK::2.0.0::0aff6865bed3a66b-DLEP
pub fn handle_lock(raw_parts: Vec<&str>, block: i64, caller: String, data: &mut ConsolidatedData) {
    let nft_to_lock = raw_parts[3];

    // Fail if collection doesn't exist
    if !data.collections.contains_key(&nft_to_lock.to_owned()) {
        data.invalid.push(Invalid {
            op_type: String::from("LOCK"),
            block: block,
            caller: caller,
            object_id: nft_to_lock.to_string(),
            message: String::from(format!("[LOCK] NFT doesn't exist: {}", nft_to_lock)),
        });
        return;
    };

    // Fail if caller isn't issuer of collection
    if data.collections.get(nft_to_lock).unwrap().issuer != caller {
        data.invalid.push(Invalid {
            op_type: String::from("LOCK"),
            block: block,
            caller: caller.clone(),
            object_id: nft_to_lock.to_string(),
            message: String::from(format!(
                "[LOCK] Caller {} doesn't own {}",
                caller, nft_to_lock
            )),
        });
        return;
    };

    //TODO create LOCK functionality.  not defined in current consolidator
    data.invalid.push(Invalid {
        op_type: String::from("LOCK"),
        block: block,
        caller: caller.clone(),
        object_id: nft_to_lock.to_string(),
        message: String::from(format!(
            "[LOCK] LOCK should be successful but logic is not implemented in code",
        )),
    });
    return;
}
