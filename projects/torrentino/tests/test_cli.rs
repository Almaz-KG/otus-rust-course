use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn no_torrent_file() {
    Command::cargo_bin("torrentino")
        .unwrap()
        .args(&[
            "-f", "no_torrent_file",
            "-t", "1",
            "-o", "target"
        ])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn no_torrent_file_specified() {
    Command::cargo_bin("torrentino")
        .unwrap()
        .args(&[
            "-t", "1",
            "-o", "target"
        ])
        .assert()
        .failure()
        .code(1);
}


