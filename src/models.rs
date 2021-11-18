use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

pub use super::base::BaseConsolidated;
pub use super::create::CreateConsolidated;
pub use super::mint::NftConsolidated;
// pub use super::resadd::ResourceConsolidated;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsolidatedData {
    pub nfts: HashMap<String, NftConsolidated>,
    pub collections: HashMap<String, CreateConsolidated>,
    pub bases: HashMap<String, BaseConsolidated>,
    pub invalid: Vec<Invalid>,
    pub last_block: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Call {
    pub caller: String,
    pub call: String,
    pub value: String,
    pub extras: Option<Vec<Call>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawRemark {
    pub block: i64,
    pub calls: Vec<Call>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Remark {
    pub protocol: String,
    pub method: String,
    pub version: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Invalid {
    pub op_type: String,
    pub block: i64,
    pub caller: String,
    pub object_id: String,
    pub message: String,
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
