use serde_derive::{Deserialize, Serialize};

pub use crate::models::{ConsolidatedData, Invalid, Remark};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResAdd {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceConsolidated {
    pub pending: bool,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<String>>,
}

// Done: NFT must exist
// Done: fail if not owner of collection
// Done: pending if not owner of NFT

// rmrk :: ACCEPT :: 2.0.0 :: 5105000-0aff6865bed3a66b-DLEP-DL15-00000001 :: NFT :: 5105000-0aff6865bed3a66b-DLEP-DL15-00000002
// rmrk :: ACCEPT :: 2.0.0 :: 5105000-0aff6865bed3a66b-DLEP-DL15-00000001 :: RES :: V1i6B
pub fn handle_accept(
    raw_parts: Vec<&str>,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    println!("accepting");
    let accepting_nft = raw_parts[3];
    let acceptance_type = raw_parts[4];
    let accepted_id = raw_parts[5];

    if acceptance_type == "NFT" {
        handle_accept_nft(accepting_nft, accepted_id, block, caller, data);
    } else if acceptance_type == "RES" {
        handle_accept_res(accepting_nft, accepted_id, block, caller, data);
    }
}

pub fn handle_accept_nft(
    accepting_nft: &str,
    accepted_nft: &str,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    // Check accepting NFT exists
    if !data.nfts.contains_key(&accepting_nft.to_owned()) {
        data.invalid.push(Invalid {
            op_type: String::from("RESADD"),
            block: block,
            caller: caller,
            object_id: accepting_nft.to_string(),
            message: String::from(format!("[RESADD] NFT doesn't exist: {}", accepting_nft)),
        });
        return;
    };

    // Check accepted NFT exists
    if !data.nfts.contains_key(&accepted_nft.to_owned()) {
        data.invalid.push(Invalid {
            op_type: String::from("RESADD"),
            block: block,
            caller: caller,
            object_id: accepted_nft.to_string(),
            message: String::from(format!("[RESADD] NFT doesn't exist: {}", accepted_nft)),
        });
        return;
    };

    // Check caller owners accepting NFT
    if data.nfts.get(accepting_nft).unwrap().rootowner != caller {
        data.invalid.push(Invalid {
            op_type: String::from("RESADD"),
            block: block,
            caller: caller.clone(),
            object_id: accepting_nft.to_string(),
            message: String::from(format!(
                "[RESADD] Caller {} doesn't own {}",
                caller, accepting_nft
            )),
        });
        return;
    };

    // Update pending value for accepted NFT
    data.nfts.entry(accepted_nft.to_string()).and_modify(|i| {
        i.pending = false;
    });

    // Update pending value for child of accepting NFT (if that child exists)
    data.nfts.entry(accepting_nft.to_string()).and_modify(|i| {
        for j in i.children.iter_mut() {
            if j.id == accepted_nft {
                j.pending = false;
                return;
            }
        }
        data.invalid.push(Invalid {
            op_type: String::from("ACCEPT"),
            block: block,
            caller: caller.clone(),
            object_id: accepting_nft.to_string(),
            message: String::from(format!(
                "[ACCEPT] Acceptor {} has no child {}",
                accepting_nft, accepted_nft
            )),
        });
        return;
    });
}

pub fn handle_accept_res(
    accepting_nft: &str,
    accepted_nft: &str,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    // Check accepting NFT exists
    if !data.nfts.contains_key(&accepting_nft.to_owned()) {
        data.invalid.push(Invalid {
            op_type: String::from("RESADD"),
            block: block,
            caller: caller,
            object_id: accepting_nft.to_string(),
            message: String::from(format!("[RESADD] NFT doesn't exist: {}", accepting_nft)),
        });
        return;
    };

    // Check caller owners accepting NFT
    if data.nfts.get(accepting_nft).unwrap().rootowner != caller {
        data.invalid.push(Invalid {
            op_type: String::from("RESADD"),
            block: block,
            caller: caller.clone(),
            object_id: accepting_nft.to_string(),
            message: String::from(format!(
                "[RESADD] Caller {} doesn't own {}",
                caller, accepting_nft
            )),
        });
        return;
    };

    // Update pending value for child of accepting NFT (if that child exists)
    data.nfts.entry(accepting_nft.to_string()).and_modify(|i| {
        for j in i.resources.iter_mut() {
            if j.id == accepted_nft {
                j.pending = false;
            };
            i.priority.push(accepted_nft.to_string());
            return;
        }
        data.invalid.push(Invalid {
            op_type: String::from("ACCEPT"),
            block: block,
            caller: caller.clone(),
            object_id: accepting_nft.to_string(),
            message: String::from(format!(
                "[ACCEPT] Acceptor {} has no resource {}",
                accepting_nft, accepted_nft
            )),
        });
        return;
    });
}
