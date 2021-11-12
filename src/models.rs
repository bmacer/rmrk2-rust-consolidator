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
    pub invalid: Vec<String>, //TODO fix this not sure what it's a vec of
    pub lastBlock: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Call {
    pub caller: String,
    pub call: String,
    pub value: String,
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
