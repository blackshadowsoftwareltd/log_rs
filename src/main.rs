use config::log_config;
use log::{error, info, warn};

pub mod config;
fn main() {
    log_config();
    error!("{}", "This is an error message");
    info!("{:?}", "This is an info message");
    warn!("{:#?}", "This is a warning message");
}
