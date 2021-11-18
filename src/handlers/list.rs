pub use crate::models::{Change, ConsolidatedData, Invalid, Remark};

// Fail if NFT doesn't exist
// Fail if NFT isn't rootowned by caller

// rmrk::LIST::2.0.0::5105000-0aff6865bed3a66b-VALHELLO-POTION_HEAL-00000001::10000000000
pub fn handle_list(raw_parts: Vec<&str>, block: i64, caller: String, data: &mut ConsolidatedData) {
    let nft_to_list = raw_parts[3];
    let price = raw_parts[4];

    // Fail if NFT doesn't exist
    if !data.nfts.contains_key(&nft_to_list.to_owned()) {
        data.invalid.push(Invalid {
            op_type: String::from("LIST"),
            block: block,
            caller: caller,
            object_id: nft_to_list.to_string(),
            message: String::from(format!("[LIST] NFT doesn't exist: {}", nft_to_list)),
        });
        return;
    };

    // Fail if caller isn't rootowner of NFT
    if data.nfts.get(nft_to_list).unwrap().rootowner != caller {
        data.invalid.push(Invalid {
            op_type: String::from("LIST"),
            block: block,
            caller: caller.clone(),
            object_id: nft_to_list.to_string(),
            message: String::from(format!(
                "[BURN] Caller {} doesn't own {}",
                caller, nft_to_list
            )),
        });
        return;
    };

    let mut d = data.nfts.get_mut(nft_to_list).unwrap();
    let old_price = d.forsale.clone();

    // Change forsale price
    d.forsale = price.to_string();

    // Add change audit log
    d.changes.push(Change {
        field: String::from("forsale"),
        old: old_price,
        new: price.to_string(),
        caller: caller.clone(),
        block: block,
        opType: String::from("LIST"),
    });
}
