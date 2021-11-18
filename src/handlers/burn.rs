pub use crate::models::{ConsolidatedData, Invalid, Remark};

// Fail if NFT doesn't exist
// Fail if NFT isn't rootowned by caller
// Burn all descendents

//TODO handle BURN reason.  maybe populate BURN field with the reason?

// rmrk :: BURN :: 2.0.0 :: 5105000-0aff6865bed3a66b-VALHELLO-POTION_HEAL-00000001
pub fn handle_burn(raw_parts: Vec<&str>, block: i64, caller: String, data: &mut ConsolidatedData) {
    let nft_to_burn = raw_parts[3];
    // Fail if NFT doesn't exist
    if !data.nfts.contains_key(&nft_to_burn.to_owned()) {
        data.invalid.push(Invalid {
            op_type: String::from("BURN"),
            block: block,
            caller: caller,
            object_id: nft_to_burn.to_string(),
            message: String::from(format!("[BURN] NFT doesn't exist: {}", nft_to_burn)),
        });
        return;
    };

    // Fail if caller doesn't own NFT
    if data.nfts.get(nft_to_burn).unwrap().rootowner != caller {
        data.invalid.push(Invalid {
            op_type: String::from("BURN"),
            block: block,
            caller: caller.clone(),
            object_id: nft_to_burn.to_string(),
            message: String::from(format!(
                "[BURN] Caller {} doesn't own {}",
                caller, nft_to_burn
            )),
        });
        return;
    };

    let mut burned_family = Vec::<String>::new();

    recursive_delete(nft_to_burn.to_string(), data, &mut burned_family);
    println!("burned family: {:?}", burned_family);
    for member in burned_family {
        data.nfts.get_mut(&member).unwrap().burned = String::from("true");
    }
}

pub fn recursive_delete(parent: String, data: &mut ConsolidatedData, list: &mut Vec<String>) {
    let children = data.nfts.get(&parent).unwrap().children.clone();
    list.push(parent);
    if children.len() == 0 {
        return;
    }
    for child in children.iter() {
        recursive_delete(child.id.clone(), data, list)
    }
}
