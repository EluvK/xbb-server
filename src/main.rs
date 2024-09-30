mod db;
mod model;
mod opt;
mod router;

use salvo::{
    conn::{
        rustls::{Keycert, RustlsConfig},
        TcpListener,
    },
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
    // read cert and key file
    let cert = std::fs::read(&config.cert).expect("cannot read cert file");
    let key = std::fs::read(&config.key).expect("cannot read key file");

    let port = config.port.unwrap_or(DEFAULT_PORT);
    let address = format!("0.0.0.0:{}", port);

    let ssl_config = RustlsConfig::new(Keycert::new().cert(cert).key(key));
    let acceptor = TcpListener::new(address).rustls(ssl_config).bind().await;

    Server::new(acceptor)
        .serve(Service::new(router::router()).hoop(ForceHttps::new().https_port(port)))
        .await;

    Ok(())
}
