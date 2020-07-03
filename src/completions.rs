use anyhow::Result;

#[path = "cli.rs"]
mod cli;

use clap_generate::{generate, generators::{Bash, Elvish, Fish, PowerShell, Zsh}};

pub fn completions(shell: Option<&str>) -> Result<()> {
    match shell {
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
