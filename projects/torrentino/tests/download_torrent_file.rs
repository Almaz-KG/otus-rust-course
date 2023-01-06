use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn download_torrent_file() {
    Command::cargo_bin("torrentino")
        .unwrap()
        .args(&[
            "-f",
            "resources/test_file_one_tracker.torrent",
            "-t",
            "1",
            "-o",
            "target",
        ])
        .assert()
        .failure()
        .code(1);
}
