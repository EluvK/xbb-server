mod db;
mod model;
mod opt;
mod router;
mod error;

use std::path::Path;

use salvo::{
    conn::{
        rustls::{Keycert, RustlsConfig},
        TcpListener,
    },
    logging::Logger,
    prelude::ForceHttps,
    Listener, Server, Service,
};

const DEFAULT_PORT: u16 = 15443;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read config.json file
    db::init_db()?;
    let opt = std::fs::read_to_string("config.json").expect("cannot read config file");
    let config = serde_json::from_str::<opt::Config>(&opt)?;

    // log file

    let log_path_str = config.log_path.unwrap_or("./".into());
    let log_path = Path::new(&log_path_str);
    let _g = file_log(log_path, false)?;

    // read cert and key file
    let cert = std::fs::read(&config.cert).expect("cannot read cert file");
    let key = std::fs::read(&config.key).expect("cannot read key file");

    let port = config.port.unwrap_or(DEFAULT_PORT);
    let address = format!("0.0.0.0:{}", port);

    let ssl_config = RustlsConfig::new(Keycert::new().cert(cert).key(key));
    let acceptor = TcpListener::new(address).rustls(ssl_config).bind().await;

    Server::new(acceptor)
        .serve(Service::new(router::router()).hoop(ForceHttps::new().https_port(port)).hoop(Logger::new()))
        .await;

    Ok(())
}

fn file_log(path: &Path, enable_debug: bool) -> anyhow::Result<impl Drop> {
    let file_path = path.join("logs");
    println!("logs file to: {file_path:?}");
    let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("xbb-server-logs")
        .filename_suffix("log")
        .build(file_path)?;
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);
    let mut subscriber = tracing_subscriber::fmt()
        .with_writer(non_blocking_appender)
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_ansi(false);
    if enable_debug {
        subscriber = subscriber.with_max_level(tracing::Level::DEBUG);
    }
    tracing::subscriber::set_global_default(subscriber.finish()).unwrap();
    tracing::info!("start");

    Ok(guard)
}
