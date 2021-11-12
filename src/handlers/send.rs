pub use crate::mint::NftConsolidated;
pub use crate::models::{ConsolidatedData, Remark};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildConsolidated {
    pub id: String,
    pub pending: bool,
    pub equipped: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Change {
    pub field: String,
    pub old: String,
    pub new: String,
    pub caller: String,
    pub block: i64,
    pub opType: String,
}

pub fn handleSend(
    resource: String,
    recipient: String,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    let mut good = false;
    let mut old = String::new();
    let mut new = String::new();

    let d: Option<&mut NftConsolidated> = data.nfts.get_mut(&resource);
    match d {
        Some(v) => {
            if caller == v.rootowner {
                good = true;
                old = v.owner.clone();
                new = recipient.clone();
                v.owner = recipient.clone();
                v.changes.push(Change {
                    field: String::from("owner"),
                    old: old,
                    new: new,
                    caller: caller,
                    block: block,
                    opType: String::from("SEND"),
                })
            } else {
                println!("caller does not equal root owner: {:?}", resource);
            }
        }
        None => {
            println!("no value found for this resource");
        }
    }
    if data.nfts.contains_key(&recipient) && good == true {
        data.nfts.entry(recipient).and_modify(|r| {
            r.children.push(ChildConsolidated {
                equipped: String::new(),
                id: resource,
                pending: false,
            });
        });
    }
}
