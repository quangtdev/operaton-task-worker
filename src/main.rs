/// Operaton Task worker is a program that periodically fetches open service tasks from the
/// Operaton instance at Energy Lab and then will try to solve them if any function or sub program
/// is available.

mod structures;

use config::Config;
use structures::ConfigParams;

use log::{debug, error, warn, log_enabled, info, Level};

fn main() {
    // Get the parameters from the environment variables
    let config = load_config();

    env_logger::init();

    info!("Load Operaton Task Worker with configuration: {:#?}", config);

    if config.username().is_empty() || config.password().is_empty() {
        warn!("No authentication set up. Operaton should be protected by authentication in productive use.");
    }

    info!("Enter the main loop");

    loop {
        info!("Test");
        main_loop(config);
        break;
    }
}

fn main_loop(config: ConfigParams) {
    let mut url = config.url().clone();
    url.set_path("engine-rest/external-task");
    info!("Fetch data at {}", url.to_string());

    
}

/// Loads the configuration into a [ConfigParams] struct. The function may panic, but it should not
/// happen because [ConfigParams] provides default values for all configured entries.
fn load_config() -> ConfigParams {
    let settings = Config::builder()
        .add_source(config::Environment::with_prefix("OTW"))
        .build()
        .unwrap();

    settings.try_deserialize().unwrap()
}