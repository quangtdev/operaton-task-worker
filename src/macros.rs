#[macro_export]
macro_rules! task_handler {
    ($name:expr, $func:path) => {
        inventory::submit! {
            crate::registry::Handler {
                name: $name,
                func: $func,
            }
        }
    };
}
