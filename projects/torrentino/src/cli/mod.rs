mod cli_args;

pub use cli_args::Arguments;

use crate::engine::TorrentEngine;
use crate::protocol::entities::Torrent;
use std::convert::TryFrom;

pub struct Cli {
    args: Arguments,
}

impl Cli {
    pub fn new(args: Arguments) -> Self {
        Cli { args }
    }

    fn check_file_existence(&self) -> Result<(), String> {
        let file = self.args.file.as_path();
        if !file.exists() {
            return Err("Torrent file doesn't exists".to_string());
        }

        if !file.is_file() {
            return Err("Provided file is a directory".to_string());
        }

        Ok(())
    }

    fn parse_torrent_file(&self) -> Result<Torrent, String> {
        let file_path = self
            .args
            .file
            .as_os_str()
            .to_str()
            .ok_or_else(|| "Unable create file path".to_string())?;
        let file_path = file_path.to_string();

        let torrent =
            Torrent::try_from(file_path).map_err(|e| format!("Unable parse torrent file {}", e))?;

        Ok(torrent)
    }

    pub fn process(&self) -> Result<(), String> {
        self.check_file_existence()?;

        let torrent = self.parse_torrent_file()?;
        let mut torrent_engine = TorrentEngine::start();

        torrent_engine.add_new_torrent(torrent);

        Ok(())
    }
}
