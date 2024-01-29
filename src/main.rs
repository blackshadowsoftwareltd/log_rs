use config::log_config;
use log::{error, info, warn};
use serde_json::{json, Value};

pub mod config;
fn main() {
    log_config();
    error!("{}", "This is an error message");
    info!("{:?}", "This is an info message");
    warn!("{:#?}", "This is a warning message");
    pretty_json();
}

fn pretty_json() {
    let data = json!({ "key1": "value1", "key2": { "nested_key": "nested_value" } });

    log_pretty_json(&data);
}
fn log_pretty_json(data: &Value) {
    if let Ok(pretty_json) = serde_json::to_string_pretty(data) {
        info!("{}", pretty_json);
    } else {
        error!("Failed to serialize data as pretty JSON");
    }
}
