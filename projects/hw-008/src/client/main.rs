mod app;
mod clients;
mod tui_gui;

use clap::Parser;
use clients::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ClientArgs {
    /// The server host to connect
    #[arg(long, value_name = "host")]
    pub host: String,

    /// The server port to connect
    #[arg(short = 'p', long)]
    pub port: u16,
}

fn main() {
    let args = ClientArgs::parse();

    let host = args.host;
    let port = args.port;

    let tcp_client = TcpClient::new(host.clone(), port);
    let udp_client = UdpClient::new(host.clone(), port);
    println!("Connecting to: {} {}", host, port);
    tui_gui::run(tcp_client, udp_client).unwrap();
}
