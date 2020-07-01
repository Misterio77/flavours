extern crate anyhow;
use anyhow::Result;

extern crate clap;
extern crate clap_generate;

#[path = "cli.rs"]
mod cli;

use clap_generate::{generate, generators::{Bash, Elvish, Fish, PowerShell, Zsh}};

pub fn completions(arguments: &clap::ArgMatches) -> Result<()> {
    match arguments.value_of("shell") {
        Some("bash") => Ok(generate::<Bash, _>(
            &mut cli::build_cli(), "flavours", &mut std::io::stdout()
        )),
        Some("elvish") => Ok(generate::<Elvish, _>(
            &mut cli::build_cli(), "flavours", &mut std::io::stdout()
        )),
        Some("fish") => Ok(generate::<Fish, _>(
            &mut cli::build_cli(), "flavours", &mut std::io::stdout()
        )),
        Some("powershell") => Ok(generate::<PowerShell, _>(
            &mut cli::build_cli(), "flavours", &mut std::io::stdout()
        )),
        Some("zsh") => Ok(generate::<Zsh, _>(
            &mut cli::build_cli(), "flavours", &mut std::io::stdout()
        )),
        _ => Ok(())
    }
}
