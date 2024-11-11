pub mod android;
pub mod config;

#[cfg(test)]
mod tests {
    use crate::config::{self, init_log_config};
    use config::get_current_dir_path;
    use log::{error, info, warn};

    #[test]
    fn test_function() {
        let path = get_current_dir_path();
        init_log_config(path);
        error!("{}", "This is an error message");
        info!("{:?}", "This is an info message");
        warn!("{:#?}", "This is a warning message");
    }
}
