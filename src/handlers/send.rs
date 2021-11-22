pub use crate::mint::NftConsolidated;
pub use crate::models::{Change, ConsolidatedData, Invalid, Remark};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildConsolidated {
    pub id: String,
    pub pending: bool,
    pub equipped: String,
}

// rmrk::SEND::{version}::{id}::{recipient}
pub fn handle_send(raw_parts: Vec<&str>, block: i64, caller: String, data: &mut ConsolidatedData) {
    let resource = raw_parts[3].to_string();
    let recipient = raw_parts[4].to_string();

    let mut pending = false;
    let mut recipient_is_nft = false;

    // Fail if NFT doesn't exist
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

    // Fail if caller isn't rootowner of NFT
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

    // Check if the recipient is an NFT.  This will affect if we update just the owner (if NFT) or the rootowner as well (if *not* NFT)
    if data.nfts.contains_key(&recipient) {
        // Recipient is NFT (update owner field only)
        recipient_is_nft = true;
        if data.nfts.get(&recipient).unwrap().rootowner == caller {
            // Recipient NFT is owned by sender, update owner field and audit log change
            let mut d = data.nfts.get_mut(&resource).unwrap();
            let old_owner = d.owner.clone();
            d.owner = recipient.clone();
            d.changes.push(Change {
                field: String::from("owner!!"),
                old: old_owner,
                new: recipient.clone(),
                caller: caller.clone(),
                block: block,
                op_type: String::from("SEND"),
            });
        } else {
            // Recipient NFT is not owned by sender (update owner field *with pending*)
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
                op_type: String::from("SEND"),
            });
        }
    } else {
        // Recipient is non-NFT address (update both owner and rootowner and add audit log entry)
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
            op_type: String::from("SEND"),
        });
    }

    // Update the recipient's children field (if recipient is NFT)
    if recipient_is_nft {
        let d = data.nfts.get_mut(&recipient).unwrap();
        d.children.push(ChildConsolidated {
            id: resource.clone(),
            pending: pending,
            equipped: String::new(),
        })
    }
}
