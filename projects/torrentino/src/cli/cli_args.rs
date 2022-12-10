use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Name or path for the .torrent file
    #[arg(short, long, value_name = "FILE")]
    pub file: PathBuf,

    /// The thread number for downloading torrent files in parallel
    #[arg(short, long, default_value_t = 1, value_name = "THREAD NUMBER")]
    pub threads: usize,

    /// A regexp like selector for selecting specific part of torrents. It might be helpful in
    /// cases, where you want to download specific file(s), instead of downloading the full torrent.
    #[arg(short, long, value_name = "REGEXP EXPRESSION")]
    pub select: Option<String>,

    /// Allows you to exclude some files from the torrent file list. It  might be used with
    /// [select] option as well.
    #[arg(short, long, value_name = "REGEXP EXPRESSION")]
    pub exclude: Option<String>,

    /// The output folder for storing downloaded files from torrent. The default value for this
    /// parameter is set to the current command location
    #[arg(short, long, default_value = ".", value_name = "OUTPUT FOLDER")]
    pub output: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Arguments::command().debug_assert()
    }
}
