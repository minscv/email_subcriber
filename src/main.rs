use std::net::TcpListener;
use email_subcriber::{run, configuration::get_configuration};
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
// Bubble up the io::Error if we failed to bind the address
// Otherwise call .await on our Server

    let configuration = get_configuration().expect("Fialed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)
        .expect("Failed to bind random port");

    run(listener)?.await
}