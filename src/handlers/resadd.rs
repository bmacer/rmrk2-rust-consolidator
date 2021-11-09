use serde_derive::{Deserialize, Serialize};

pub use crate::models::{ConsolidatedData, Remark};

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
}

pub fn handle_resadd(
    res: String,
    r: Remark,
    _block: i64,
    _caller: String,
    data: &mut ConsolidatedData,
) {
    let u = urlencoding::decode(&r.value).unwrap().into_owned();
    let dec: Result<ResAdd, serde_json::Error> = serde_json::from_str(&u);
    match dec {
        Ok(v) => {
            let consolidated = ResourceConsolidated {
                pending: false,
                base: v.base,
                id: v.id,
                license: v.license,
                metadata: v.metadata,
                slot: v.slot,
                src: v.src,
                thumb: v.thumb,
            };
            let d = data.nfts.contains_key(&res);
            if d {
                let d = data.nfts.entry(res);
                d.and_modify(|i| i.resources.push(consolidated));
            } else {
                println!("ERROR DOESN'T CONTAIN KEY, resAdd! {:?}", r);
                //TODO handle this error (add to invalid)
            }
        }
        Err(e) => {
            println!("Error in redAdd: {:?} {:?}", e, u);
        }
    }
}
