use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

use roster::ServerConfigBuilder;
mod port_picker;
use port_picker::pick_unused_port;

/// Start a simple Roster server
pub fn start_simple_server() -> SocketAddr {
    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        pick_unused_port().unwrap(),
    );
    let server_config = ServerConfigBuilder::default()
        .connections_limit(Arc::new(20.into()))
        .bind_addr(addr)
        .build()
        .unwrap();
    let _handle = std::thread::spawn(move || {
        server_config.initialize();
        
    });
    addr
}

pub fn debug_server() -> SocketAddr {
    SocketAddr::from_str("127.0.0.1:3456").unwrap()
}

pub async fn connect_without_auth(
    addr: SocketAddr,
) -> redis_async::client::PairedConnection {
    use tokio::time::Duration;

    tokio::time::sleep(Duration::from_secs(2)).await;

    redis_async::client::paired_connect(addr.ip().to_string(), addr.port())
        .await
        .unwrap()
}
