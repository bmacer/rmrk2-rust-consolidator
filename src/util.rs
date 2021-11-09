use super::models;
use serde_json;

pub fn parse_line(line: &str) -> Result<models::RawRemark, serde_json::Error> {
    let l = serde_json::from_str(&line);
    return l;
}
