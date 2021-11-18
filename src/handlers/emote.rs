pub use crate::models::{Call, Change, ConsolidatedData, Invalid, Remark};
pub use crate::util::subkey_inspect;

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum Error {
    Int(ParseIntError),
    Unicode(u32),
}

fn parse_unicode(input: &str) -> Result<char, Error> {
    let unicode = u32::from_str_radix(input, 16).map_err(Error::Int)?;
    char::from_u32(unicode).ok_or_else(|| Error::Unicode(unicode))
}

// Fail if emote is not valid unicode
// For RMRK2 only
//   Fail if NFT doesn't exist
//   Insert new reaction if emote doesn't currently exist
//   If new reaction exists, add caller to list if caller is not currently on list
//   If caller is on list, remove them from the list (unemote)

// TODO handle RMRK1 and other namespaces?

// RMRK::EMOTE::2.0.0::RMRK2::5105000-0aff6865bed3a66b-DLEP-DL15-00000001::1F389

pub fn handle_emote(raw_parts: Vec<&str>, block: i64, caller: String, data: &mut ConsolidatedData) {
    let namespace = raw_parts[3];
    let emote_recipient = raw_parts[4];
    let emote = raw_parts[5];
    if parse_unicode(emote).is_err() {
        data.invalid.push(Invalid {
            op_type: String::from("EMOTE"),
            block: block,
            caller: caller,
            object_id: emote.to_string(),
            message: String::from(format!("[EMOTE] Invalid Unicode: {}", emote)),
        });
        return;
    }
    if namespace == "RMRK2" || namespace == "rmrk2" {
        // Fail if NFT doesn't exist
        if !data.nfts.contains_key(emote_recipient) {
            data.invalid.push(Invalid {
                op_type: String::from("EMOTE"),
                block: block,
                caller: caller,
                object_id: emote_recipient.to_string(),
                message: String::from(format!("[EMOTE] non-existent NFT {}", emote_recipient)),
            });
            return;
        }
        data.nfts
            .entry(emote_recipient.to_string())
            .and_modify(|i| {
                // If emote doesn't exist, insert and add caller to it
                if !i.reactions.contains_key(emote) {
                    i.reactions.insert(emote.to_string(), vec![caller]);
                } else {
                    i.reactions.entry(emote.to_string()).and_modify(|i| {
                        match i.iter().position(|x| *x == caller) {
                            // If caller is not on list for emote, add caller
                            None => i.push(caller.clone()),
                            // If caller is on list for emote, remove caller
                            Some(v) => {
                                i.remove(v);
                            }
                        }
                    });
                }
            });
    }
}
