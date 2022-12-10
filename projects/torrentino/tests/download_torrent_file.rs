use torrentino::cli::{Arguments, Cli};

#[test]
fn download_torrent_file(){
    let args = Arguments {
        file: "resources/test_file.torrent".to_string().parse().unwrap(),
        threads: 1,
        select: None,
        exclude: None,
        output: Some("target".to_string())
    };


    let cli = Cli::new(args);
    let result = cli.process();
    println!("{:?}", result);

    assert!(result.is_ok());



}