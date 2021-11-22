pub use crate::models::{Call, Change, ConsolidatedData, Invalid, Remark};
pub use crate::util::subkey_inspect;

// Done: Fail if NFT doesn't exist
// Fail if NFT isn't listed
// Fail if NFT is burned
// Fail if there is no corresponding balance transfer
// Fail if there is no corresponding balance transfer to rootowner
// Change owner of NFT
// Change price to 0
// Add changes record for owner
// Add changes record for price

//TODO add recursive rootowner of children when purchasing?

// rmrk::BUY::2.0.0::5105000-0aff6865bed3a66b-VALHELLO-POTION_HEAL-00000001

pub fn handle_buy(
    raw_parts: Vec<&str>,
    extras: Option<Vec<Call>>,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    let nft_to_buy = raw_parts[3];
    let mut nft_recipient = caller.clone();
    if raw_parts.len() == 5 {
        nft_recipient = raw_parts[4].to_string();
    }

    // Fail if NFT doesn't exist
    if !data.nfts.contains_key(nft_to_buy) {
        data.invalid.push(Invalid {
            op_type: String::from("BUY"),
            block: block,
            caller: caller,
            object_id: nft_to_buy.to_string(),
            message: String::from(format!("[BUY] non-existent NFT {}", nft_to_buy)),
        });
        return;
    }

    // Fail if NFT isn't listed
    if data.nfts.get(nft_to_buy).unwrap().forsale == "0" {
        data.invalid.push(Invalid {
            op_type: String::from("BUY"),
            block: block,
            caller: caller,
            object_id: nft_to_buy.to_string(),
            message: String::from(format!("[BUY] NFT is not LISTed {}", nft_to_buy)),
        });
        return;
    }
    let price_listed = &data.nfts.get(nft_to_buy).unwrap().forsale.clone();
    let nft_seller = &data.nfts.get(nft_to_buy).unwrap().rootowner.clone();
    let kus_nft_seller = subkey_inspect(nft_seller.to_string()).unwrap_or(String::new());

    // Fail if NFT is burned
    if data.nfts.get(nft_to_buy).unwrap().burned != "" {
        data.invalid.push(Invalid {
            op_type: String::from("BUY"),
            block: block,
            caller: caller,
            object_id: nft_to_buy.to_string(),
            message: String::from(format!("[BUY] Cannot buy BURNed NFT {}", nft_to_buy)),
        });
        return;
    }

    // Fail if there is no corresponding balance transfer
    let mut found_qualifying_balance_transfer = false;
    match extras {
        None => {
            data.invalid.push(Invalid {
                op_type: String::from("BUY"),
                block: block,
                caller: caller,
                object_id: nft_to_buy.to_string(),
                message: String::from(format!(
                    "[BUY] No balance transfer found.  Should be {} sent to {}",
                    price_listed, kus_nft_seller
                )),
            });
            return;
        }
        Some(extra) => {
            for ex in extra {
                if ex.call == "balances.transfer" {
                    if ex.value.contains(",") {
                        let split: Vec<&str> = ex.value.split(",").collect();
                        let balance_receiver = split[0];
                        let sent_amount = split[1];
                        let kus_balance_receiver =
                            subkey_inspect(balance_receiver.to_string()).unwrap_or(String::new());
                        if kus_balance_receiver.len() > 1
                            && kus_balance_receiver == kus_nft_seller
                            && sent_amount == price_listed
                        {
                            found_qualifying_balance_transfer = true
                        }
                    }
                }
            }
        }
    }
    if !found_qualifying_balance_transfer {
        data.invalid.push(Invalid {
            op_type: String::from("BUY"),
            block: block,
            caller: caller,
            object_id: nft_to_buy.to_string(),
            message: String::from(format!(
                "[BUY] No proper balance transfer found.  Should be {} sent to {}",
                price_listed, kus_nft_seller
            )),
        });
        return;
    }

    // Change owner of NFT
    let mut d = data.nfts.get_mut(nft_to_buy).unwrap();
    let old_owner = d.owner.clone();
    d.owner = nft_recipient.clone();
    d.rootowner = nft_recipient.clone();

    // Change forsale price to 0
    d.forsale = String::from("0");

    // Add change audit log for forsale price
    d.changes.push(Change {
        field: String::from("forsale"),
        old: price_listed.to_string(),
        new: String::from("0"),
        caller: caller.clone(),
        block: block,
        op_type: String::from("BUY"),
    });

    // Add change audit log for owner
    d.changes.push(Change {
        field: String::from("owner"),
        old: old_owner,
        new: nft_recipient,
        caller: caller.clone(),
        block: block,
        op_type: String::from("BUY"),
    });
}

/*

{"block":11,"calls":[{"call":"system.remark",
"value":"0x524d524b3a3a4255593a3a322e302e303a3a352d414c494345535f434f4c4c454354494f4e2d414c494345535f4e46542d3030313a3a466f514a70507961645963636a6176566454577870785537725545615968664c43507758676b6644365a6174395150",
"caller":"FoQJpPyadYccjavVdTWxpxU7rUEaYhfLCPwXgkfD6Zat9QP",
"extras":[{
    "call":"balances.transfer",
    "value":"15oF4uVJwmo4TdGW7VfQxNLavjCXviqxT9S1MgbjMNHr6Sp5,1000000000000",
    "caller":"FoQJpPyadYccjavVdTWxpxU7rUEaYhfLCPwXgkfD6Zat9QP"}]}]}


*/
