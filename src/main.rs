use std::io::Error;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080))
        .await
        .expect("Failed to bind a port");

    zero2prod::run(listener).await
}
