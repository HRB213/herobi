use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

mod gencomp;
mod category;

#[derive(Parser, Debug)]
#[command(
    name = "herobi",
    version,
    about = "ディレクトリ内の構成を素早く把握するCLIツール"
)]
pub struct Cli {
    /// ファイルを種類ごとに分類
    #[arg(short, long)]
    category: bool,

    /// ディレクトリサイズを表示
    #[arg(short, long)]
    size: bool,

    /// サマリーを表示
    #[arg(short = 'm', long)]
    summary: bool,

    #[arg(
        long,
        help = "Generate shell completion files",
        default_value_t = false
    )]
    completions: bool,

    /// 対象ディレクトリ
    path: Option<PathBuf>,
}

#[derive(Default)]
struct Summary {
    directories: usize,
    files: usize,
    total_size: u64,
}

fn main() {
    let cli = Cli::parse();

    if cli.completions {
        if let Err(e) = gencomp::generate_completions() {

            eprintln!("{}", e);

            process::exit(1);

        }
        return;
    }

    let path = cli.path.unwrap_or_else(|| PathBuf::from("."));

    if !path.exists() {
        eprintln!("Error: directory does not exist.");
        process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("Error: path is not a directory.");
        process::exit(1);
    }

    let mut entries = Vec::new();

    let read_dir = match fs::read_dir(&path) {
        Ok(rd) => rd,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    for entry in read_dir {
        if let Ok(entry) = entry {
            entries.push(entry);
        }
    }

    entries.sort_by_key(|e| e.file_name());

    if !cli.category && !cli.size && !cli.summary {
        print_normal(&entries);
    }

    if cli.category {
        print_category(&entries);
    }

    if cli.size {
        print_sizes(&entries);
    }

    if cli.summary {
        print_summary(&path);
    }
}

fn print_normal(entries: &[fs::DirEntry]) {
    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        // 隠しファイル・隠しディレクトリは表示しない
        if name.starts_with('.') {
            continue;
        }
        println!("{}", name);
    }
}

fn print_category(entries: &[fs::DirEntry]) {
    use category::Category;

    let mut directories = Vec::new();
    let mut rust = Vec::new();
    let mut markdown = Vec::new();
    let mut image = Vec::new();
    let mut audio = Vec::new();
    let mut video = Vec::new();
    let mut archive = Vec::new();
    let mut other = Vec::new();

    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();

        // 隠しファイル・隠しディレクトリは除外
        if name.starts_with('.') {
            continue;
        }

        let path = entry.path();

        if path.is_dir() {
            directories.push(name);
            continue;
        }

        match category::classify(&path) {
            Category::Rust => rust.push(name),
            Category::Markdown => markdown.push(name),
            Category::Image => image.push(name),
            Category::Audio => audio.push(name),
            Category::Video => video.push(name),
            Category::Archive => archive.push(name),
            Category::Other => other.push(name),
        }
    }

    if !directories.is_empty() {
        println!("Directories");
        println!("-----------");
        for item in directories {
            println!("{}", item);
        }
        println!();
    }

    if !rust.is_empty() {
        println!("Rust");
        println!("----");
        for item in rust {
            println!("{}", item);
        }
        println!();
    }

    if !markdown.is_empty() {
        println!("Markdown");
        println!("--------");
        for item in markdown {
            println!("{}", item);
        }
        println!();
    }

    if !image.is_empty() {
        println!("Image");
        println!("-----");
        for item in image {
            println!("{}", item);
        }
        println!();
    }

    if !audio.is_empty() {
        println!("Audio");
        println!("-----");
        for item in audio {
            println!("{}", item);
        }
        println!();
    }

    if !video.is_empty() {
        println!("Video");
        println!("-----");
        for item in video {
            println!("{}", item);
        }
        println!();
    }

    if !archive.is_empty() {
        println!("Archive");
        println!("-------");
        for item in archive {
            println!("{}", item);
        }
        println!();
    }

    if !other.is_empty() {
        println!("Other");
        println!("-----");
        for item in other {
            println!("{}", item);
        }
        println!();
    }
}

fn print_sizes(entries: &[fs::DirEntry]) {
    println!("Directory Size");
    println!("--------------");

    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') {
            continue;
        }

        let path = entry.path();

        if path.is_dir() {
            let size = dir_size(&path);

            println!("{:<20} {}", name, human_size(size));
        }
    }

    println!();
}

fn print_summary(root: &Path) {
    let mut summary = Summary::default();

    collect_summary(root, &mut summary);

    println!("Summary");
    println!("-------");
    println!("Directories : {}", summary.directories);
    println!("Files       : {}", summary.files);
    println!("Total Size  : {}", human_size(summary.total_size));
}

fn collect_summary(path: &Path, summary: &mut Summary) {
    let read_dir = match fs::read_dir(path) {
        Ok(r) => r,
        Err(_) => return,
    };

    for entry in read_dir.flatten() {
        let p = entry.path();

        if p.is_dir() {
            summary.directories += 1;
            collect_summary(&p, summary);
        } else {
            summary.files += 1;

            if let Ok(meta) = entry.metadata() {
                summary.total_size += meta.len();
            }
        }
    }
}

fn dir_size(path: &Path) -> u64 {
    let mut total = 0;

    let read_dir = match fs::read_dir(path) {
        Ok(r) => r,
        Err(_) => return 0,
    };

    for entry in read_dir.flatten() {
        let p = entry.path();

        if p.is_dir() {
            total += dir_size(&p);
        } else if let Ok(meta) = entry.metadata() {
            total += meta.len();
        }
    }

    total
}

fn human_size(size: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let s = size as f64;

    if s >= GB {
        format!("{:.1} GB", s / GB)
    } else if s >= MB {
        format!("{:.1} MB", s / MB)
    } else if s >= KB {
        format!("{:.1} KB", s / KB)
    } else {
        format!("{} B", size)
    }
}