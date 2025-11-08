use config::Config;

use crate::structures::ConfigParams;

/// The prefix for all environment variables used by Operaton Task Worker
///
/// Note: This does not apply for Rust-specific environment variables such as `LOGLEVEL`.
pub const ENV_PREFIX: &str = "OPERATON_TASK_WORKER";

/// Loads the configuration into a [ConfigParams] struct. The function may panic, but it should not
/// happen because [ConfigParams] provides default values for all configured entries.
pub fn load_config() -> ConfigParams {
    let settings = Config::builder()
        .add_source(config::Environment::with_prefix(ENV_PREFIX))
        .build()
        .unwrap();

    settings.try_deserialize().unwrap()
}
