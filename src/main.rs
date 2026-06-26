use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process;
mod gencomp;

/// ファイルやディレクトリの一覧を表示するCLIツール
#[derive(Parser, Debug)]
#[command(
    name = "herobi",
    version,
    about = "ファイルやディレクトリの一覧を表示するCLIツール",
    long_about = "指定したディレクトリ内のファイルおよびディレクトリを一覧表示します。"
)]

struct Cli {

    #[arg(default_value = ".")]

    path: PathBuf,

    #[arg(

        long,

        help = "Generate shell completion files",

        default_value_t = false

    )]

    completions: bool,

}

fn main() {
    let cli = Cli::parse();

    if cli.completions {
    gencomp::generate_completions().unwrap();
    return;
    }

    if let Err(err) = list_directory(&cli.path) {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}

fn list_directory(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path)? {
        entries.push(entry?);
    }

    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(())
}