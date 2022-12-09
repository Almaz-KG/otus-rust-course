use crate::cli::Arguments;
use crate::protocol::entities::Torrent;
use std::convert::TryFrom;

pub struct Cli {
    args: Arguments,
}

impl Cli {
    pub fn new(args: Arguments) -> Self {
        Cli { args }
    }

    fn check_file_existence(&self) -> Result<(), String>{
        let file = self.args.file.as_path();
        if !file.exists() {
            return Err("Torrent file doesn't exists".to_string())
        }

        if !file.is_file() {
            return Err("Provided file is a directory".to_string())
        }

        Ok(())
    }

    fn parse_torrent_file(&self) -> Result<Torrent, String> {
        let file_path = self.args.file.as_os_str().to_str()
            .ok_or("Unable create file path".to_string())?;
        let file_path = file_path.to_string();

        Ok(Torrent::try_from(file_path)
            .map_err(|e| format!("Unable parse torrent file {}", e))?)
    }

    pub fn process(&self) -> Result<(), String> {
        println!("Processing with {:?}", self.args);
        self.check_file_existence()?;

        let torrent = self.parse_torrent_file()?;

        println!("{:?}", torrent.info.name);

        Ok(())
    }
}