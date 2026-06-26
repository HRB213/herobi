use clap::CommandFactory;
use clap_complete::{
    generate_to,
    shells::{Bash, Fish, PowerShell, Zsh},
};
use std::fs;
use std::path::Path;

use crate::Cli;

/// シェル補完ファイルを生成する
pub fn generate_completions() -> Result<(), Box<dyn std::error::Error>> {
    let outdir = Path::new("completions");

    fs::create_dir_all(outdir)?;

    let mut cmd = Cli::command();

    generate_to(Bash, &mut cmd, "herobi", outdir)?;
    generate_to(Zsh, &mut cmd, "herobi", outdir)?;
    generate_to(Fish, &mut cmd, "herobi", outdir)?;
    generate_to(PowerShell, &mut cmd, "herobi", outdir)?;

    println!("Completion files were generated in {:?}", outdir);

    Ok(())
}