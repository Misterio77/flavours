use crate::cli::build_cli;
use anyhow::Result;
use std::io;

use clap_generate::{
    generate,
    generators::{Bash, Elvish, Fish, PowerShell, Zsh},
};

pub fn completions(shell: Option<&str>) -> Result<()> {
    match shell {
        Some("bash") => generate::<Bash, _>(&mut build_cli(), "flavours", &mut io::stdout()),
        Some("elvish") => generate::<Elvish, _>(&mut build_cli(), "flavours", &mut io::stdout()),
        Some("powershell") => {
            generate::<PowerShell, _>(&mut build_cli(), "flavours", &mut io::stdout())
        },
        Some("fish") => {
            let mut buffer = Vec::new();
            generate::<Fish, _>(&mut build_cli(), "flavours", &mut buffer);
            let buffer = String::from_utf8(buffer)?;
            println!("{}", buffer.replace("placeholder", "(flavours list -l)"));
        }
        Some("zsh") => {
            let mut buffer = Vec::new();
            generate::<Zsh, _>(&mut build_cli(), "flavours", &mut buffer);
            let buffer = String::from_utf8(buffer)?;
            println!("{}", buffer.replace("(placeholder)", "($(flavours list))"));
        }
        _ => {}
    };
    Ok(())
}
