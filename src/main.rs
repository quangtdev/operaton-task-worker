/// Operaton Task Worker is a program that periodically fetches open service tasks from the
/// Operaton instance at Energy Lab and then will try to solve them if any function or sub program
/// is available.

use operaton_task_worker::{poll, settings};
use operaton_task_worker_macros::task_handler;

/// The prefix for all environment variables used by Operaton Task Worker
///
/// Note: This does not apply for Rust-specific environment variables such as `LOGLEVEL`.
pub const ENV_PREFIX: &str = "OPERATON_TASK_WORKER";

#[tokio::main]
async fn main() {
    // Get the parameters from the environment variables
    let config = settings::load_config_from_env(ENV_PREFIX);
    poll(config).await;
}

#[task_handler(name = "ServiceTask_Grant_Approval")]
fn service_task_grant_approval(_input: &operaton_task_worker::types::InputVariables) -> Result<operaton_task_worker::types::OutputVariables, Box<dyn std::error::Error>> {
    Ok(std::collections::HashMap::new())
}