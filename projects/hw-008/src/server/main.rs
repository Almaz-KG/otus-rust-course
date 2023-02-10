use clap::Parser;
use hw_008::server::{TcpServer, UdpServer};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ServerArgs {
    /// An optional server host. If no id provided it will be considered as localhost
    #[arg(long, value_name = "host", default_value = "localhost")]
    pub host: Option<String>,

    /// An optional server port. If no port provided it will be generated randomly
    #[arg(short, long, value_name = "port")]
    pub port: Option<u16>,
}

fn main() {
    let args = ServerArgs::parse();

    println!("Starting servers with {:?}", args);

    let host = args.host.unwrap_or_else(|| "localhost".into());
    let port = args.port.unwrap_or(0u16);
    let current_dir = std::env::current_dir().expect("Unable determine the current dir");

    let tcp_server = TcpServer::start(host.clone(), port);
    UdpServer::start(host, port + 1, current_dir);

    tcp_server.join().unwrap()
}
