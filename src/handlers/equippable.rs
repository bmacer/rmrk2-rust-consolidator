pub use crate::mint::NftConsolidated;
pub use crate::models::{ConsolidatedData, Invalid, Remark, EquippableOption};
use log::warn;

// Fail if base doesn't exist
// Fail if caller isn't the issuer of the base
// Fail if slot doesn't exist on base
// Determine if it is add, subtract, or override operation

// rmrk::EQUIPPABLE::2.0.0::base-575878273-kanaria_epic_birds::wing_slot_1::+0aff6865bed3a66b-FOO

pub fn handle_equippable(
    raw_parts: Vec<&str>,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    let resource = raw_parts[3].to_string();
    let slot = raw_parts[4].to_string();
    let changes_raw = raw_parts[5].to_string();

    // Fail if base doesn't exist
    if !data.bases.contains_key(&resource) {
        data.invalid.push(Invalid {
            op_type: String::from("EQUIPPABLE"),
            block: block,
            caller: caller,
            object_id: resource.clone(),
            message: String::from(format!(
                "[EQUIPPABLE] Base doesn't exist: {}",
                resource.clone()
            )),
        });
        return;
    };

    // Fail if caller isn't the issuer of the base
    if data.bases.get(&resource).unwrap().issuer != caller {
        data.invalid.push(Invalid {
            op_type: String::from("EQUIPPABLE"),
            block: block,
            caller: caller.clone(),
            object_id: resource.clone(),
            message: String::from(format!(
                "[EQUIPPABLE] Caller {} isn't issuer of base {}",
                caller,
                resource.clone()
            )),
        });
        return;
    };

    let first_char = changes_raw.chars().next().unwrap_or(' ');

    data.bases.entry(resource.clone()).and_modify(|i| {
        for part in &mut i.parts {
            if part.part_type == String::from("slot") {
                if part.id == slot {
                    if first_char == ' ' {
                        warn!("not sure if empty value should blank out the list, but it's not explicitly stated so just returning");
                        return;
                    }
                    // Override * for the whole list.
                    //TODO the only way to remove this would be with a -* (or overrride) which seems improper
                    if first_char == '*' {
                        part.equippable = Some(EquippableOption::All);
                    }
                    let mut string = changes_raw.chars();
                    string.next();
                    let new_string = string.as_str();
                    let mut to_add: Vec<&str> = new_string.split(",").collect();
                    // Add values if + used
                    if first_char == '+' {
                        for item in to_add.iter() {
                            match &mut part.equippable {
                                Some(v) => {
                                    match v {
                                        EquippableOption::All => (),
                                        EquippableOption::OneCollection(c) => {
                                            if c != &item.to_string() {
                                                part.equippable = Some(EquippableOption::Collections(vec![c.to_string(), item.to_string()]));
                                            }
                                        }
                                        EquippableOption::Collections(c) => {
                                            if !c.contains(&item.to_string()) {
                                                c.push(item.to_string())
                                            }
                                        }
                                    }
                                    
                                }
                                None => part.equippable = Some(
                                    EquippableOption::Collections(vec![item.to_string()])
                                ),
                            }
                        }
                        return;
                    }
                    // Subtract values if - used
                    if first_char == '-' {
                        for item in to_add.iter() {
                            match &mut part.equippable {
                                Some(v) => {
                                    match v {
                                        EquippableOption::All => {
                                            data.invalid.push(Invalid {
                                                op_type: String::from("EQUIPPABLE"),
                                                block: block,
                                                caller: caller.clone(),
                                                object_id: resource.clone(),
                                                message: String::from(format!(
                                                    "[EQUIPPABLE] Cannot subtract from wildcard * equippable.  Caller {} isn't issuer of base {}",
                                                    caller,
                                                    resource.clone()
                                                )),
                                            });
                                            return;
                                        }
                                        EquippableOption::OneCollection(c) => {
                                            if c == &item.to_string() {
                                                part.equippable = Some(EquippableOption::Collections(vec![]));
                                            } else {
                                                data.invalid.push(Invalid {
                                                    op_type: String::from("EQUIPPABLE"),
                                                    block: block,
                                                    caller: caller.clone(),
                                                    object_id: resource.clone(),
                                                    message: String::from(format!(
                                                        "[EQUIPPABLE] Collection {} not in equippables list.  Caller {} isn't issuer of base {}",
                                                        c,
                                                        caller,
                                                        resource.clone()
                                                    )),
                                                });
                                                return;
                                            }
                                        }
                                        EquippableOption::Collections(c) => {
                                            if c.contains(&item.to_string()) {
                                                let index = c.iter().position(|x| *x == item.to_string()).unwrap();
                                                c.remove(index);
                                            }
                                        }
                                    }
                                }
                                None => (),
                            }
                        }
                        return;
                    }
                
                    to_add = changes_raw.split(",").collect();
                        // Override if no other options
                        part.equippable = Some(EquippableOption::Collections(vec![]));
                        for item in to_add.iter() {
                            
                            match &mut part.equippable {
                                Some(v) => {
                                    match v {
                                        EquippableOption::All => {
                                            data.invalid.push(Invalid {
                                                op_type: String::from("EQUIPPABLE"),
                                                block: block,
                                                caller: caller.clone(),
                                                object_id: resource.clone(),
                                                message: String::from(format!(
                                                    "[EQUIPPABLE] Cannot add to wildcard * equippable.  Caller {} isn't issuer of base {}",
                                                    caller,
                                                    resource.clone()
                                                )),
                                            });
                                            return;
                                        }
                                        EquippableOption::OneCollection(_) => {
                                            part.equippable = Some(EquippableOption::Collections(vec![item.to_string()]));
                                        }
                                        EquippableOption::Collections(c) => {
                                            if !c.contains(&item.to_string()) {
                                                c.push(item.to_string());
                                            }
                                        }
                                    }
                                },
                                None => part.equippable = Some(EquippableOption::Collections(vec![item.to_string()])),
                            }
                        }
                    }
                }
            }
        });
    }
