use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;

use tokio::net::TcpListener;
use zero2prod::startup;

pub struct App {
    address: SocketAddr,
}

impl App {
    pub async fn new() -> Self {
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .await
            .expect("Failed to start an app in test");
        let address = listener.local_addr().unwrap();
        tokio::spawn(startup::run(listener));
        App { address }
    }

    pub fn url(&self, path: &str) -> String {
        format!("http://{}/{}", self.address, path.trim_start_matches("/"))
    }
}
