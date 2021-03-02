use crate::cli::build_cli;
use anyhow::Result;
use std::io::stdout;

use clap_generate::{
    generate,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};

pub fn completions(shell: Option<&str>) -> Result<()> {
    match shell {
        Some("bash") => generate::<Bash, _>(&mut build_cli(), "flavours", &mut stdout()),
        Some("elvish") => generate::<Elvish, _>(&mut build_cli(), "flavours", &mut stdout()),
        Some("fish") => generate::<Fish, _>(&mut build_cli(), "flavours", &mut stdout()),
        Some("powershell") => {
            generate::<PowerShell, _>(&mut build_cli(), "flavours", &mut stdout())
        }
        Some("zsh") => generate::<Zsh, _>(&mut build_cli(), "flavours", &mut stdout()),
        _ => {}
    };
    Ok(())
}
