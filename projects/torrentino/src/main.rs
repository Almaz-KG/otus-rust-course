#![allow(dead_code)]
#![allow(unused_imports)]

use torrentino::cli::{Arguments, Cli};

use bincode::Options;
use clap::Parser;
use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};
use std::thread;
use torrentino::protocol::entities::ConnectionRequest;

fn main() -> Result<(), String> {
    let arguments = Arguments::parse();

    // let arguments = Arguments {
    //     file: "resources/test_file.torrent".to_string().parse().unwrap(),
    //     threads: 1,
    //     select: None,
    //     exclude: None,
    //     output: Some("target".to_string()),
    // };

    let cli = Cli::new(arguments);
    cli.process()
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     println!("{:?}", "tracker.leechers-paradise.org:6969"
//         .to_socket_addrs());
//
//     let remote_addr: SocketAddr = "tracker.leechers-paradise.org:6969"
//         .to_socket_addrs()
//         .unwrap()
//         .as_slice()[0];
//
//
//     // We use port 0 to let the operating system allocate an available port for us.
//     let local_addr: SocketAddr = if remote_addr.is_ipv4() {
//         "0.0.0.0:0"
//     } else {
//         "[::]:0"
//     }
//     .parse()?;
//
//     // 000004172710198000000000edb0f0b9
//
//     let socket = UdpSocket::bind(local_addr).await?;
//     const MAX_DATAGRAM_SIZE: usize = 65_507;
//     socket.connect(&remote_addr).await?;
//
//     let cr = ConnectionRequest::new();
//     let cr_code = bincode::serialize(&cr).unwrap();
//     println!("{:?}", &cr_code);
//     // let cr_code = bincode::options().with_big_endian().serialize(&cr).unwrap();
//     // println!("{:?}", &cr_code);
//
//     to
//     for i in 0..10 {
//         socket.send(&cr_code).await?;
//         std::thread::sleep(std::time::Duration::from_millis(10));
//         println!("SENT");
//     }
//
//
//     let mut data = vec![0u8; MAX_DATAGRAM_SIZE];
//     let len = socket.recv(&mut data).await?;
//     println!(
//         "Received {} bytes:\n{}",
//         len,
//         String::from_utf8_lossy(&data[..len])
//     );
//
//     Ok(())
// }
