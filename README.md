<p align="center">
  <img src="https://raw.githubusercontent.com/Misterio77/flavours/master/logo.svg" alt="flavours logo"/>
</p>

---

A (WIP) manager/builder for [Base16](https://github.com/chriskempson/base16). Written in Rust🦀.

## What
The program is designed with three operations in mind: update, query, apply.

Update will handle downloading base16 sources, schemes and templates.
Query will return a list of available schemes matching the argument.
Apply will (through a simple user configuration file) take a scheme, apply it to the apps configuration files the user specified, and run specified hooks for updating each application. If the user supplies a pattern that matches more than one scheme, the program should chose one randomly.

Additionaly, there's a 'completions' operation, that will generate completion-scripts for specified shell. I will probably pack these scripts when releasing packages for distros.

## Why
I started this project because the existing tools didn't seem mature enough or didn't quite fit my workflow.
I made a (quite functional) [bash script](https://gist.github.com/Misterio77/925243bdf3e7ce1f23654507e5326f7a), but i wanted something faster, modular and that could be used by other people without having to be rewritten each time. I've been wanting to try and make an actual rust project, so here we are.

## How
I'm using this opportunity to learn rust, so it's fully written in it. [clap](https://github.com/clap-rs/clap) is used for cli arguments, [glob](https://github.com/rust-lang-nursery/glob) for pattern matching and [anyhow](https://github.com/dtolnay/anyhow) for error handling.

## When
I plan to have it fully functional by the end of 2020 July, but i will take my sweet time to make it better and featureful. Feel free to suggest changes, open issues or PR, i'm very open to feedback <3


### Thanks to:
- Functionality inspiration from [Base16 Universal Manager](https://github.com/pinpox/base16-universal-manager).
- Logo inspiration from [Starship prompt](https://starship.rs)
