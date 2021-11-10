pub use crate::mint::NftConsolidated;
pub use crate::models::{ConsolidatedData, Remark};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChildConsolidated {
    pub id: String,
    pub pending: bool,
    pub equipped: String,
}

pub fn handleEquip(
    resource: String,
    slot: String,
    _block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    let mut owner = String::new();
    match data.nfts.get(&resource) {
        Some(v) => {
            owner = v.owner.clone();
        }
        None => println!("handleEquip error: no resource found for {:?}", resource),
    }

    match data.nfts.get_mut(&owner) {
        Some(o) => {
            for mut child in o.children.iter_mut() {
                // println!("child: {:?}", child);
                if child.id == resource {
                    child.equipped = slot;
                    return;
                }
                println!("no child resource found for {:?} in {:?}", resource, owner);
            }
        }
        None => println!("no parent resource found: {:?}", owner),
    }
}
