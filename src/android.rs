extern crate log;
use std::env;

#[cfg(target_os = "android")]
use android_logger::{Config, FilterBuilder};
use log::trace;
#[cfg(target_os = "android")]
use log::LevelFilter;

pub fn init_android_log() {
    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );

    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Error),
    );

    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Warn),
    );
}

pub fn android_trace(m: &str) {
    match env::consts::OS {
        "android" => trace!("Android Error: {}", m),
        _ => {}
    }
}

pub fn android_error(m: &str) {
    match env::consts::OS {
        "android" => log::error!("Android Error: {}", m),
        _ => {}
    }
}

pub fn android_info(m: &str) {
    match env::consts::OS {
        "android" => log::info!("Android Info: {}", m),
        _ => {}
    }
}

pub fn android_warn(m: &str) {
    match env::consts::OS {
        "android" => log::warn!("Android Warn: {}", m),
        _ => {}
    }
}
