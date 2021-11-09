pub use crate::mint::NftConsolidated;
pub use crate::models::{ConsolidatedData, Remark};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildConsolidated {
    pub id: String,
    pub pending: bool,
    pub equipped: String,
}

pub fn handleSend(
    resource: String,
    recipient: String,
    _block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    let d: Option<&mut NftConsolidated> = data.nfts.get_mut(&resource);
    match d {
        Some(v) => {
            if caller == v.rootowner {
                let recipient_nft: Option<&mut NftConsolidated> = data.nfts.get_mut(&recipient);
                match recipient_nft {
                    Some(r) => {
                        r.children.push(ChildConsolidated {
                            equipped: String::new(),
                            id: resource,
                            pending: false,
                        });
                    }
                    None => {
                        println!("Recipient doesn't exist. {:?}", recipient);
                    }
                }
            } else {
                println!("caller does not equal root owner: {:?}", resource);
            }
        }
        None => {
            println!("no value found for this resource");
        }
    }
}
