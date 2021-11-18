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

pub fn handle_resadd(
    res: String,
    r: Remark,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    // Fail if NFT doesn't exist
    if !data.nfts.contains_key(&res) {
        data.invalid.push(Invalid {
            op_type: String::from("RESADD"),
            block: block,
            caller: caller,
            object_id: res.clone(),
            message: String::from(format!("[RESADD] NFT doesn't exist: {}", res)),
        });
        return;
    };
    let u = urlencoding::decode(&r.value).unwrap().into_owned();
    let dec: Result<ResAdd, serde_json::Error> = serde_json::from_str(&u);
    match dec {
        Ok(v) => {
            // Fail if caller is not owner of collection
            let collection_name = data.nfts.get(&res).unwrap().collection.clone();

            if !data.collections.contains_key(&collection_name) {
                data.invalid.push(Invalid {
                    op_type: String::from("RESADD"),
                    block: block,
                    caller: caller,
                    object_id: r.value.clone(),
                    message: String::from(format!(
                        "[RESADD] Collection name doesn't exist: {}",
                        collection_name
                    )),
                });
                return;
            }
            let owner_of_collection = data
                .collections
                .get(&collection_name)
                .unwrap()
                .issuer
                .clone();
            if caller != owner_of_collection {
                // Fails if caller is not issuer of the NFT's collection
                data.invalid.push(Invalid {
                    op_type: String::from("RESADD"),
                    block: block,
                    caller: caller.clone(),
                    object_id: r.value,
                    message: String::from(format!(
                        "[RESADD] Caller {} is not owner of collection {}",
                        caller, owner_of_collection
                    )),
                });
                return;
            }
            let owner_of_nft = data.nfts.get(&res).unwrap().rootowner.clone();

            // Will be pending if owner of NFT is not the caller
            let pending = owner_of_nft != caller;
            let consolidated = ResourceConsolidated {
                pending: pending,
                base: v.base,
                id: v.id.clone(),
                license: v.license,
                metadata: v.metadata,
                slot: v.slot,
                src: v.src,
                thumb: v.thumb,
                parts: v.parts,
            };

            let d = data.nfts.entry(res);
            d.and_modify(|i| {
                i.resources.push(consolidated);
                i.priority.push(v.id);
            });
        }
        Err(e) => {
            data.invalid.push(Invalid {
                op_type: String::from("RESADD"),
                block: block,
                caller: caller,
                object_id: r.value,
                message: String::from(format!("[RESADD] Missing values: {}", e)),
            });
        }
    }
}
