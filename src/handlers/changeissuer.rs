pub use crate::models::{Call, Change, ConsolidatedData, Invalid, Remark};
pub use crate::util::subkey_inspect;

// Fail if Resource or Collection doesn't exist
// Fail if Resource/Collection issuer isn't caller
// Update issuer
// Add change audit log for issuer

// rmrk::CHANGEISSUER::2.0.0::0aff6865bed3a66b-DLEP::HviHUSkM5SknXzYuPCSfst3CXK4Yg6SWeroP6TdTZBZJbVT

pub fn handle_changeissuer(
    raw_parts: Vec<&str>,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    let affected_base_or_collection = raw_parts[3];
    let new_issuer = raw_parts[4];
    let base_exists = data.bases.contains_key(affected_base_or_collection);
    let collection_exists = data.collections.contains_key(affected_base_or_collection);
    if !base_exists && !collection_exists {
        data.invalid.push(Invalid {
            op_type: String::from("CHANGEISSUER"),
            block: block,
            caller: caller,
            object_id: affected_base_or_collection.to_string(),
            message: String::from(format!(
                "[CHANGEISSUER] No base or collection with id: {}",
                affected_base_or_collection
            )),
        });
        return;
    }
    if base_exists {
        if data
            .bases
            .get(affected_base_or_collection)
            .unwrap()
            .issuer
            .clone()
            != caller
        {
            data.invalid.push(Invalid {
                op_type: String::from("CHANGEISSUER"),
                block: block,
                caller: caller.clone(),
                object_id: affected_base_or_collection.to_string(),
                message: String::from(format!(
                    "[CHANGEISSUER] {} is not current issuer of base {}",
                    caller, affected_base_or_collection
                )),
            });
            return;
        }

        let d = data.bases.get_mut(affected_base_or_collection).unwrap();

        let old_issuer = d.issuer.clone();
        d.issuer = new_issuer.to_string();

        d.changes.push(Change {
            field: String::from("issuer"),
            old: old_issuer,
            new: new_issuer.to_string(),
            caller: caller.clone(),
            block: block,
            op_type: String::from("CHANGEISSUER"),
        });
        return;
    }
    if collection_exists {
        if data
            .collections
            .get(affected_base_or_collection)
            .unwrap()
            .issuer
            != caller
        {
            data.invalid.push(Invalid {
                op_type: String::from("CHANGEISSUER"),
                block: block,
                caller: caller.clone(),
                object_id: affected_base_or_collection.to_string(),
                message: String::from(format!(
                    "[CHANGEISSUER] {} is not current issuer of collection {}",
                    caller, affected_base_or_collection
                )),
            });
            return;
        }
        let d = data
            .collections
            .get_mut(affected_base_or_collection)
            .unwrap();

        let old_issuer = d.issuer.clone();
        d.issuer = new_issuer.to_string();

        d.changes.push(Change {
            field: String::from("issuer"),
            old: old_issuer,
            new: new_issuer.to_string(),
            caller: caller.clone(),
            block: block,
            op_type: String::from("CHANGEISSUER"),
        });
    }
}
