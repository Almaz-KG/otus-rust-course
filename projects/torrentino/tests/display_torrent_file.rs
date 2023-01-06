use std::convert::TryFrom;
use std::path::PathBuf;
use torrentino::protocol::entities::Torrent;

fn display_torrent_file(file: PathBuf) {
    let torrent = Torrent::try_from(file).expect("Unable parse torrent file");
    println!("{}", torrent);
}

#[test]
fn display_torrents() {
    let file1: PathBuf = "resources/test_file_one_tracker.torrent"
        .to_string()
        .parse()
        .unwrap();
    // let file1: PathBuf = "resources/test_file.torrent".to_string().parse().unwrap();
    // let file2: PathBuf = "resources/ubuntu.torrent".to_string().parse().unwrap();

    display_torrent_file(file1);
    // display_torrent_file(file2);
}
