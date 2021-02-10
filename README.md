<p align="center">
  <img src="https://raw.githubusercontent.com/Misterio77/flavours/master/logo.svg" alt="flavours logo"/>
</p>

---

A manager/builder for [Base16](https://github.com/chriskempson/base16). Written in Rust🦀.

[![Crates.io](https://img.shields.io/crates/v/flavours.svg)](https://crates.io/crates/flavours)
[![Build](https://github.com/misterio77/flavours/workflows/Rust/badge.svg)](https://github.com/misterio77/flavours/actions?query=workflow%3ARust)

[![Packaging status](https://repology.org/badge/vertical-allrepos/flavours.svg)](https://repology.org/project/flavours/versions)


## What
This is a CLI program that both builds and manages Base16 schemes and templates.

The base16 specification consists of both schemes (with 16 colors) and templates. There'll probably be templates for most applications you use, but you can easily make one for literally any app that supports any sort of color customization.

Once your configuration files are set, you can theme your entire desktop with just *one* command. No more hassle changing themes when you get bored.
Why have one color if you can have all the colors?

I use it to theme kitty terminal, i3wm, rofi, polybar, gtk, discord and spotify. Here's how it looks in action:
![Usage](http://u.cubeupload.com/Misterio77x/flavours202009191033.gif)

## How

### Installation

#### Packages
- [AUR Package](https://aur.archlinux.org/packages/flavours/) for Arch (and derivatives):
`yay -S flavours`

Let me know if you want to package flavours for your favorite distro.

#### Cargo
Just install cargo and run `cargo install flavours` (don't forget to include `~/.cargo/bin` on your PATH).

#### Post-install
After installing, you should probably use `flavours update all` to grab all published schemes and templates from the base16 repos. If you want, you can manually tweak the templates, schemes or even the repo lists (everything's located in `~/.local/share/flavours` on Linux, can be changed using the `FLAVOURS_DATA_DIRECTORY` global variable).

### Usage
You can use flavours and base16 templates to automatically inject schemes into any application config file that supports colors codes.

#### Setup
Choose a [template](https://github.com/chriskempson/base16#template-repositories) for each app you want themed (or create your own).

On these config files, place a start and end comment to tell flavours where to **replace** lines (defaults are `# Start flavours` and `# End flavours`). These usually should be located where you set color options on your app configuration. If the specific app supports including colors from another file, or if the template provides the entire file, you can forgo the comments altogether and use the `rewrite=true` on flavours config.

For reference, here's a couple configuration files from my [dots](https://github.com/Misterio77/dotfiles):
- [zathura](https://github.com/Misterio77/dotfiles/blob/master/home/.config/zathura/zathurarc)
- [dunst](https://github.com/Misterio77/dotfiles/blob/master/home/.config/dunst/dunstrc)
- [polybar](https://github.com/Misterio77/dotfiles/blob/master/home/.config/polybar/config.ini)
- [alacritty](https://github.com/Misterio77/dotfiles/tree/master/home/.config/alacritty/alacritty.yml) 
- [rofi](https://github.com/Misterio77/dotfiles/blob/master/home/.config/rofi/themes/styles/colors.rasi) (rewrite mode)


On flavours configuration (`~/.config/flavours/config.toml` on Linux, can be changed using the `FLAVOURS_CONFIG_FILE` global variable):
- Create a `[[item]]` section for each app, each section can have the following entries:
  - Specify the `file` to write (required)
  - A `template` (required)
  - A `subtemplate` (for when the template has one other than "default")
  - A `hook` to execute (do keep in mind this currently **doesn't** go through your shell, so if you want to use bash syntax, do it like this: `hook='bash -c "my cool && bash stuff"'`)
  - Whether to use `rewrite` mode (if you do, you don't need the start and end comments)
  - Or change the `start` and `end` lines (useful for config files which comments are not started with `#`)

Vóila. You're now ready to apply schemes.

#### Applying
`flavours apply` is the command you'll probably be using all the time. So it's built to be as useful as possible.

The syntax is `flavours apply [PATTERN]`, where PATTERN can be a scheme name, multiple scheme name, a glob (such as `*light`) expression, or can be ommited.
If more than one scheme is specified or matched, flavours will choose one randomly (`flavours apply *light` will pick one random scheme ending with light, and apply it).

You can, for instance:
- Specify a scheme: `flavours apply pasque`
- Specify multiple schemes: `flavours apply pasque paraiso atlas`
- Use glob: `flavours apply "gruvbox*"`
- Omit: `flavours apply` (is the same as running `flavours apply "*"`)

#### Other commands
You can also use `flavours current` to see the last scheme you applied, `flavours list` to list all available schemes (`-l` or `--lines` to print each in one line, you can also use PATTERN like on apply to list only specific scheme(s)), `flavours info` to show info (including truecolor colored output, `r` or `--raw` to disable) about some scheme(s) (also using the PATTERN syntax).

Lastly, we have `flavours generate`, it can generate a scheme based on an image (usually your wallpaper), with the following syntax: `flavours generate <dark/light> /path/to/image/file`. By default, the scheme will be saved with the slug (the scheme name referenced in all other commands) `generated` (you can change it with `-s` or `--slug`, or output to stdout instead with `--stdout`).

In my setup, i use feh to apply wallpapers, and i can get the current wallpaper with the command `cat .fehbg | tail -1 | cut -d "'" -f2`.

So my flavours command to generate and apply a dark scheme matching my wallpaper would be:

`flavours generate dark $(cat .fehbg | tail -1 | cut -d "'" -f2) && flavours apply generated`

Which i include in the script i use to change my wallpapers randomly.

## Why
Why use this instead of other base16 managers, or even pywal?

While these projects are great and certainly fit some people's workflow, they didn't quite fit mine.

I decided to do my own project because i wanted a simple CLI program that can easily integrate with rofi, polybar, and everything else in my workflow. The main feature is being able to quickly and easily use a curated list of schemes, on all your apps. It's also objectively faster than _any_ other manager.

## When
All features are implemented! I'm currently working on improving code quality and stabilizing the features, but everything should work as intended.

### Thanks to:
- Functionality inspiration from [Base16 Universal Manager](https://github.com/pinpox/base16-universal-manager).
- Logo inspiration from [Starship prompt](https://starship.rs)
