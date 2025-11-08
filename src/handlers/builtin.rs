use crate::types::{InputVariables, OutputVariables, out_string, out_json};

#[operaton_task_worker_macros::task_handler(name = "example_echo")]
pub fn example_echo(input: &InputVariables) -> Result<OutputVariables, Box<dyn std::error::Error>> {
    let mut out: OutputVariables = std::collections::HashMap::new();
    out.insert("workerResponse".to_string(), out_string("ok"));

    // Return a summary JSON of the input variable names
    let keys: Vec<&String> = input.keys().collect();
    let summary = serde_json::json!({ "keys": keys });
    out.insert("summary".to_string(), out_json(&summary));

    Ok(out)
}

#[operaton_task_worker_macros::task_handler(name = "ServiceTask_GetScannedFiles")]
pub fn get_scanned_files(_input: &InputVariables) -> Result<OutputVariables, Box<dyn std::error::Error>> {
    let mut out: OutputVariables = std::collections::HashMap::new();
    out.insert("FILENAMES".to_string(), out_string("TEST"));
    Ok(out)
}
