pub mod config;

#[cfg(test)]
mod tests {
    use config::{get_current_dir_path, log_config};
    use log::{error, info, warn};

    use crate::config;

    #[test]
    fn test_function() {
        let path = get_current_dir_path();
        log_config(path);
        error!("{}", "This is an error message");
        info!("{:?}", "This is an info message");
        warn!("{:#?}", "This is a warning message");
    }
}
