pub use crate::models::{ConsolidatedData, Invalid, Remark};

// rmrk::THEMEADD::{version}::{base_id}::{name}::{html_encoded_json})
pub fn handle_themeadd(
    raw_parts: Vec<&str>,
    block: i64,
    caller: String,
    data: &mut ConsolidatedData,
) {
    let base = raw_parts[3].to_string();
    let _name = raw_parts[4].to_string();
    let _html_encoded_json = raw_parts[4].to_string();

    // Fail if base doesn't exist
    if !data.bases.contains_key(&base) {
        data.invalid.push(Invalid {
            op_type: String::from("THEMEADD"),
            block: block,
            caller: caller,
            object_id: base.clone(),
            message: String::from(format!("[THEMEADD] Base doesn't exist: {}", base)),
        });
        return;
    };

    //TODO implement THEMEADD logic, for now record as invalid just for auditing
    data.invalid.push(Invalid {
        op_type: String::from("THEMEADD"),
        block: block,
        caller: caller,
        object_id: base.clone(),
        message: String::from(format!(
            "[THEMEADD] Implementation for THEMEADD doesn't exist: {}",
            base
        )),
    });
    return;
}
