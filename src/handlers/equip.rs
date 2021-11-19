pub use crate::mint::NftConsolidated;
pub use crate::models::{ConsolidatedData, Invalid, Remark};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildConsolidated {
    pub id: String,
    pub pending: bool,
    pub equipped: String,
}

// DONE Fail if NFT doesn't exist
// DONE Fail if NFT has been BURNed
// DONE Fail if NFT is not immediate parent
// DONE Fail if child is PENDING

//TODO check unequipping

// rmrk::EQUIP::2.0.0::5105000-0aff6865bed3a66b-DLEP-ARMOR-00000001::base_1.slot_1
// rmrk::EQUIP::2.0.0::5105000-0aff6865bed3a66b-DLEP-ARMOR-00000001::

pub fn handle_equip(raw_parts: Vec<&str>, block: i64, caller: String, data: &mut ConsolidatedData) {
    let resource = raw_parts[3].to_string();
    let slot = raw_parts[4].to_string();

    if !data.nfts.contains_key(&resource) {
        data.invalid.push(Invalid {
            op_type: String::from("EQUIP"),
            block: block,
            caller: caller,
            object_id: resource.clone(),
            message: String::from(format!("[EQUIP] NFT doesn't exist: {}", resource.clone())),
        });
        return;
    };

    if data.nfts.get(&resource).unwrap().burned != "" {
        data.invalid.push(Invalid {
            op_type: String::from("EQUIP"),
            block: block,
            caller: caller,
            object_id: resource.clone(),
            message: String::from(format!(
                "[EQUIP] Can't equip on BURNed NFT: {}",
                resource.clone()
            )),
        });
        return;
    };

    if data.nfts.get(&resource).unwrap().pending {
        data.invalid.push(Invalid {
            op_type: String::from("EQUIP"),
            block: block,
            caller: caller,
            object_id: resource.clone(),
            message: String::from(format!(
                "[EQUIP] Can't equip on a pending NFT: {}",
                resource.clone()
            )),
        });
        return;
    };

    let parent = data.nfts.get(&resource).unwrap().owner.clone();

    match data.nfts.get_mut(&parent) {
        Some(o) => {
            for mut child in o.children.iter_mut() {
                if child.id.clone() == resource {
                    child.equipped = slot;
                    return;
                }
            }
            data.invalid.push(Invalid {
                op_type: String::from("EQUIP"),
                block: block,
                caller: caller,
                object_id: resource.clone(),
                message: String::from(format!(
                    "[EQUIP] Child not {} found in parent: {}",
                    resource.clone(),
                    parent.clone()
                )),
            });
            return;
        }
        None => {
            data.invalid.push(Invalid {
                op_type: String::from("EQUIP"),
                block: block,
                caller: caller,
                object_id: resource.clone(),
                message: String::from(format!("[EQUIP] Parent {} NFT not found", parent.clone())),
            });
            return;
        }
    }
}
