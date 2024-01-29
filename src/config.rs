use std::path::PathBuf;

use log::LevelFilter;
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

pub fn log_config() {
    let stdout = ConsoleAppender::builder().encoder(_encoder()).build();
    let root = PathBuf::from("logs");

    let total_logs = 10;
    let roller = FixedWindowRoller::builder()
        .build(root.join("log{}.log").to_str().unwrap(), total_logs + 1)
        .unwrap();

    // let file_size = 1024 * 1024 * 10; // ? 10 MB
    let file_size = 1024 * 5; // ? 5 KB
    let size_trigger = SizeTrigger::new(file_size);
    let policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(roller));

    let requests = RollingFileAppender::builder()
        .encoder(_encoder())
        .build(root.join("log0.log"), Box::new(policy))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("requests")
                .build(LevelFilter::Info),
        )
        .unwrap();

    if let Err(err) = log4rs::init_config(config) {
        log::info!("Error initializing logrs: {}", err);
    }

    log::info!("Logrs initialized.");
}

fn _encoder() -> Box<PatternEncoder> {
    // ? set encoder to _date_time() or _utc_time() or _local_time()
    Box::new(_local_date_time())
}

fn _date() -> PatternEncoder {
    // ? 2024-01-29 14:13:04
    PatternEncoder::new("{n}{d(%Y-%m-%d)} [{l}] [{f} ~~ {L} {n}~> {m}")
}

fn _date_time() -> PatternEncoder {
    // ? 2024-01-29 14:13:04
    PatternEncoder::new("{n}{d(%Y-%m-%d %H:%M:%S)} [{l}] [{f} ~~ {L} {n}~> {m}")
}

fn _utc_time() -> PatternEncoder {
    // ? 2024-01-29T08:16:51.758199030+00:00
    PatternEncoder::new("{n}{d(%+)(utc)} [{l}] [{f} ~~ {L} {n}~> {m}")
}

fn _utc_date_time() -> PatternEncoder {
    // ? 2024-01-29T08:16:51
    PatternEncoder::new("{n}{d(%Y-%m-%d %H:%M:%S)(utc)} [{l}] [{f} ~~ {L} {n}~> {m}")
}

fn _local_time() -> PatternEncoder {
    // ? 2024-01-29T08:16:51.758199030+00:00
    PatternEncoder::new("{n}{d(%+)(local)} [{l}] [{f} ~~ {L} {n}~> {m}")
}

fn _local_date_time() -> PatternEncoder {
    // ? 2024-01-29T08:16:51
    PatternEncoder::new("{n}{d(%Y-%m-%d %H:%M:%S)(local)} [{l}] [{f} ~~ {L}] {n}~> {m}")
}
