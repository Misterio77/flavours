<p align="center">
  <img src="https://raw.githubusercontent.com/Misterio77/flavours/master/logo.svg" alt="flavours logo"/>
</p>

---

A (WIP) manager/builder for [Base16](https://github.com/chriskempson/base16). Written in Rust🦀.



## What
This program is both a manager and a builder designed for Base16 schemes and templates. It's being written in Rust, using [clap](https://github.com/clap-rs/clap), [glob](https://github.com/rust-lang-nursery/glob) and [anyhow](https://github.com/dtolnay/anyhow) as the main libraries.

This is my first time with a public project in Rust, i'm 100% open to suggestions, critics, issues and PRs!

## How
flavours' main functionality is the apply subcommand. When run, it'll take a base16 scheme, look up your flavours configuration file (which will be a toml file) and apply that scheme to all specified templates, injecting (or replacing) a file. There will be also a hook option for each program when applying, a command to run after the operation, allowing you to reload all themed apps, so your scheme changes are instantly visible.

It's designed to be the most out of your way as possible, so you can easily integrate it with rofi menus, polybar widgets, and anything you can think of.

Once all features are implemented and they're relatively stable, i will document usage on the wiki (even though it's pretty straightforward), including guides for some applications.

## Why
Why use this instead of other base16 managers, or even pywal?

While these projects are great and certainly fit some people's workflow, they didn't quite fit mine.
I decided to do my own project because i wanted a simple and straightforward way for anyone to use base16. The main objective is to have a single command that allows you to apply a specific scheme, in all your applications in the blink of an eye. With that, you have a program that easily integrates with anything you want it to.

## When
I plan to have it fully functional by the end of 2020 July, but i will take my sweet time to make it better and featureful. Feel free to report issues or help me out <3


### Thanks to:
- Functionality inspiration from [Base16 Universal Manager](https://github.com/pinpox/base16-universal-manager).
- Logo inspiration from [Starship prompt](https://starship.rs)
