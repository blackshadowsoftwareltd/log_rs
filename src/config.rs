use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

pub fn log_config() {
    let stdout = ConsoleAppender::builder().build();
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d}[{l}]-> {m}\n")))
        // .encoder(Box::new(PatternEncoder::default()))
        .build("log/requests.log")
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
