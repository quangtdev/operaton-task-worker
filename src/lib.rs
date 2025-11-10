mod polling;
pub mod structures;
pub mod process_variables;
pub mod types;
mod api;
pub mod registry;
pub mod handlers;
pub mod settings;

pub use inventory;

use crate::structures::ConfigParams;

/// Start the polling loop asynchronously. Call this inside a Tokio runtime.
pub async fn poll(config: ConfigParams) {
    polling::start_polling_loop(config).await;
}

/// Convenience: start the polling loop and block the current thread until it ends (infinite loop).
pub fn poll_blocking(config: ConfigParams) {
    let rt = tokio::runtime::Runtime::new().expect("failed to create Tokio runtime");
    rt.block_on(async move { polling::start_polling_loop(config).await });
}
