/// Operaton Task Worker is a program that periodically fetches open service tasks from the
/// Operaton instance at Energy Lab and then will try to solve them if any function or sub program
/// is available.

use operaton_task_worker::settings;

#[tokio::main]
async fn main() {
    // Get the parameters from the environment variables
    let config = settings::load_config();


}




