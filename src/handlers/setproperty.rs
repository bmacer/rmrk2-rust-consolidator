pub use crate::mint::NftConsolidated;
pub use crate::models::{Change, ConsolidatedData, Invalid, Remark};

use serde_json::json;

// Fail if NFT doesn't exist
// Fail if caller isn't rootowner of the NFT
// Fail is property doesn't exist
// Fail is property doesn't have _mutable
// Set property (delete list and populate)

//rmrk::SETPROPERTY::2.0.0::{id}::{html_encoded_name}::{html_encoded_value}
pub fn handle_setproperty(
    raw_parts: Vec<&str>,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    let resource = raw_parts[3];
    let name = raw_parts[4];
    let value = raw_parts[5];

    // Fail if NFT doesn't exist
    if !data.nfts.contains_key(resource) {
        data.invalid.push(Invalid {
            op_type: String::from("SETPROPERTY"),
            block: block,
            caller: caller,
            object_id: resource.to_string(),
            message: String::from(format!("[SETPROPERTY] non-existent NFT {}", resource)),
        });
        return;
    }

    // Fail if caller isn't rootowner of NFT
    if data.nfts.get(resource).unwrap().rootowner != caller {
        data.invalid.push(Invalid {
            op_type: String::from("SETPROPERTY"),
            block: block,
            caller: caller.clone(),
            object_id: resource.to_string(),
            message: String::from(format!(
                "[SETPROPERTY] Caller {} does not own {}",
                caller, resource
            )),
        });
        return;
    };

    if !data
        .nfts
        .get(resource)
        .unwrap()
        .properties
        .clone()
        .unwrap()
        .clone()
        .contains_key(name)
    {
        data.invalid.push(Invalid {
            op_type: String::from("SETPROPERTY"),
            block: block,
            caller: caller.clone(),
            object_id: resource.to_string(),
            message: String::from(format!(
                "[SETPROPERTY] Key not found in properties: {}",
                resource
            )),
        });
        return;
    };
    if !data
        .nfts
        .get(resource)
        .unwrap()
        .properties
        .clone()
        .unwrap()
        .clone()
        .get(name)
        .unwrap()
        .mutation
        .clone()
        .unwrap()
        .allowed
    {
        data.invalid.push(Invalid {
            op_type: String::from("SETPROPERTY"),
            block: block,
            caller: caller.clone(),
            object_id: resource.to_string(),
            message: String::from(format!(
                "[SETPROPERTY] Key not found in properties: {}",
                resource
            )),
        });
        return;
    }

    let d = data.nfts.get_mut(resource.clone()).unwrap();

    match &mut d.properties {
        None => (),
        Some(v) => match v.get_mut(name) {
            None => (),
            Some(v) => {
                v.value
                    .insert(String::from("value"), json!(value.to_string()));
            }
        },
    }
}
