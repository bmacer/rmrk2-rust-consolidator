pub use crate::mint::NftConsolidated;
pub use crate::models::{Change, ConsolidatedData, Invalid, Remark};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildConsolidated {
    pub id: String,
    pub pending: bool,
    pub equipped: String,
}

pub fn handle_send(
    resource: String,
    recipient: String,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    let mut pending = false;
    let mut recipient_is_nft = false;

    // Check if resource exists, error if not
    if !data.nfts.contains_key(&resource) {
        data.invalid.push(Invalid {
            op_type: String::from("SEND"),
            block: block,
            caller: caller,
            object_id: resource.clone(),
            message: String::from(format!("[SEND] non-existent NFT {}", resource)),
        });
        return;
    }

    //

    // Check if sender owns resource, error if not
    if data.nfts.get(&resource).unwrap().rootowner != caller {
        data.invalid.push(Invalid {
            op_type: String::from("SEND"),
            block: block,
            caller: caller.clone(),
            object_id: resource.clone(),
            message: String::from(format!(
                "[SEND] Caller {} does not own {}",
                caller, resource
            )),
        });
        return;
    };

    if data.nfts.contains_key(&recipient) {
        recipient_is_nft = true;
        // Recipient is NFT
        if data.nfts.get(&recipient).unwrap().rootowner == caller {
            // Recipient NFT is owned by sender
            let mut d = data.nfts.get_mut(&resource).unwrap();
            let old_owner = d.owner.clone();
            d.owner = recipient.clone();
            d.changes.push(Change {
                field: String::from("owner!!"),
                old: old_owner,
                new: recipient.clone(),
                caller: caller.clone(),
                block: block,
                opType: String::from("SEND"),
            });
        } else {
            // Recipient NFT is not owned by sender (need to update owner with pending)
            pending = true;
            let recipient_root_owner = data.nfts.get(&recipient).unwrap().rootowner.clone();
            let mut d = data.nfts.get_mut(&resource).unwrap();
            let old_owner = d.owner.clone();
            d.owner = recipient.clone();
            d.pending = true;
            d.rootowner = recipient_root_owner;
            d.changes.push(Change {
                field: String::from("owner"),
                old: old_owner,
                new: recipient.clone(),
                caller: caller.clone(),
                block: block,
                opType: String::from("SEND"),
            });
        }
    } else {
        // Recipient is non-NFT address (update both owner and rootowner)
        let mut d = data.nfts.get_mut(&resource).unwrap();
        let old_owner = d.owner.clone();
        d.owner = recipient.clone();
        d.rootowner = recipient.clone();
        d.changes.push(Change {
            field: String::from("owner"),
            old: old_owner,
            new: recipient.clone(),
            caller: caller.clone(),
            block: block,
            opType: String::from("SEND"),
        });
    }

    // Update the recipient's children field
    if recipient_is_nft {
        let d = data.nfts.get_mut(&recipient).unwrap();
        d.children.push(ChildConsolidated {
            id: resource.clone(),
            pending: pending,
            equipped: String::new(),
        })
    }
}
