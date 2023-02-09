mod app;
mod clients;
mod tui_gui;
mod commands;
mod tui_example;

use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;
use clap::Parser;
use clients::*;
use crate::app::{ApplicationState, ApplicationStateUpdater};
use crate::commands::ClientCommand;

type ServerResponse = Result<String, String>;

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
    let app_state = ApplicationState::new(tcp_client, udp_client);

    let app_state_lock = Arc::new(Mutex::new(app_state));
    let (sender, receiver) = mpsc::channel::<ClientCommand>();
    let application_state_updater = ApplicationStateUpdater::new(
        app_state_lock.clone(), receiver);
    application_state_updater.start();

    tui_gui::run(app_state_lock.clone(), sender).unwrap();

    // tui_example::run();
}
