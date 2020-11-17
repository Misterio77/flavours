use crate::cli::build_cli;
use anyhow::Result;
use std::io::stdout;

use clap_generate::{
    generate,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};

pub fn completions(shell: Option<&str>) -> Result<()> {
    match shell {
        Some("bash") => Ok(generate::<Bash, _>(
            &mut build_cli(),
            "flavours",
            &mut stdout(),
        )),
        Some("elvish") => Ok(generate::<Elvish, _>(
            &mut build_cli(),
            "flavours",
            &mut stdout(),
        )),
        Some("fish") => Ok(generate::<Fish, _>(
            &mut build_cli(),
            "flavours",
            &mut stdout(),
        )),
        Some("powershell") => Ok(generate::<PowerShell, _>(
            &mut build_cli(),
            "flavours",
            &mut stdout(),
        )),
        Some("zsh") => Ok(generate::<Zsh, _>(
            &mut build_cli(),
            "flavours",
            &mut stdout(),
        )),
        _ => Ok(()),
    }
}
