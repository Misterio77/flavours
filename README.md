<p align="center">
  <img src="https://raw.githubusercontent.com/Misterio77/flavours/master/logo.svg" alt="flavours logo"/>
</p>

---

A manager/builder for [Base16](https://github.com/chriskempson/base16). Written in Rust🦀.

[![Crates.io](https://img.shields.io/crates/v/flavours.svg)](https://crates.io/crates/flavours)
[![Build](https://github.com/misterio77/flavours/workflows/Rust/badge.svg)](https://github.com/misterio77/flavours/actions?query=workflow%3ARust)

[![Packaging status](https://repology.org/badge/vertical-allrepos/flavours.svg)](https://repology.org/project/flavours/versions)

Looking for something similar to use with NixOS/home-manager? Check out [`nix-colors`](https://github.com/misterio77/nix-colors)

## What
This is a CLI program that both builds and manages Base16 schemes and templates.

The Base16 specification consists of both schemes (with 16 colors) and templates. There'll probably be templates for most applications you use, but you can easily make one for literally any app that supports any sort of color customization.

Once your configuration files are set, you can theme your entire desktop with just *one* command. No more hassle manually changing themes when you get bored.
Why have one color if you can have all the colors?

Here's how it looks in action (sway, waybar, alacritty):
![Usage](https://u.cubeupload.com/Misterio77x/ezgifcomgifmaker.gif)

## How

### Installation

#### Packages
- [AUR Package](https://aur.archlinux.org/packages/flavours/) for Arch (and derivatives):
`yay -S flavours`
- [nixpkg](https://github.com/NixOS/nixpkgs/blob/master/pkgs/applications/misc/flavours/default.nix#L17) for NixOS:
`nix-env -iA nixos.flavours`
- [Homebrew](https://formulae.brew.sh/formula/flavours) for macOS:
`brew install flavours`

Let me know if you want to package flavours for your favorite distro.

#### Cargo
Just install cargo and run `cargo install --locked flavours` (don't forget to include `~/.cargo/bin` in your PATH).

#### Post-install
After installing, you should probably use `flavours update all` to grab all published schemes and templates from the Base16 repos. By default, these files are located in `~/.local/share/flavours` on Linux, and `~/Library/Application Support/flavours` on macOS. This can be changed with the `-d`/`--directory` flag or `FLAVOURS_DATA_DIRECTORY` environment variable.

If you want to make changes to schemes/templates or make your own, see [Custom templates and schemes](#custom-templates-and-schemes).

### Usage
You can use flavours and Base16 templates to automatically inject schemes into any application config file that supports color codes.

[Dave Snider](https://www.youtube.com/channel/UC7uO9V1Frl_wPd9d1qOm_RQ) did a great [3 episode series about flavours](https://www.youtube.com/playlist?list=PL8qZ9gACSwvOxJY7nnI9CUnyCe_c1bNYO) (and theming in general). If you're into guide videos, I strongly recommend you take a look.

#### Setup
Choose a [template](https://github.com/chriskempson/base16-templates-source/blob/master/list.yaml) for each app you want themed (or create your own).

Add your apps to the flavours configuration, located at `~/.config/flavours/config.toml` on Linux and macOS. This can be changed with `-c`/`--config` flag or `FLAVOURS_CONFIG_FILE` environment variable.

For the flavours configuration file, `config.toml`:
- Optionally, set a `shell` through which your hook commands should be executed. Defaults to `sh -c '{}'`.
- Create an `[[items]]` section for each app. Each section can have the following entries:
  - The `file` to write (required).
  - A `template` (required).
  - A `subtemplate`. Defaults to `default`.
  - A `hook` to execute. Defaults to none.
  - Specified as `light`, for lightweight changes that are quick to execute. Defaults to `true`. `flavours apply --light` will skip running hooks marked with `light=false`.
  - Whether to `rewrite` the entire file instead of replacing lines. Defaults to `false`, but it is recommended to set this to true for apps that can have an entire file defining colors through import or some other means.
  - If rewrite=false, specify the `start` and `end` lines for replacing text. This is useful for config files where comments do not begin with `#`. Defaults to `# Start flavours` and `# End flavours` (case-insensitive).

Here's an example:
```toml
# Commands go through bash
shell = "bash -c '{}'"

# Sway supports the default '#' comments, so it can be ommited
# 'rewrite' is also ommited, as it defaults to false
[[items]]
file = "~/.config/sway/config"
template = "sway"
subtemplate = "colors"
hook = "swaymsg reload"
# Swaymsg reload temporarily freezes input, so it's marked as not light
light = false

# This one uses waybar 'default' subtemplate, so it can be ommited
[[items]]
file = "~/.config/waybar/colors.css"
template = "waybar"
# Waybar uses a separate color file, so we can safely rewrite the whole file
rewrite = true

[[items]]
file = "~/.config/beautifuldiscord/style.css"
template = "styles"
subtemplate = "css-variables"
# What if the configuration doesn't support '#' comments? Just change them!
start= "/* Start flavours */"
end = "/* End flavours */"
```

For files where `rewrite=false` (or omitted), tell flavours where to replace lines by placing a _start_ and _end_ comment in the app's config file where colors are set. Default _start_ and _end_ comments are `# Start flavours` and `# End flavours`.

For reference, here's a couple configuration files from my [dots](https://github.com/Misterio77/dotfiles):
- [flavours](https://github.com/Misterio77/dotfiles/blob/sway/home/.config/flavours/config.toml) itself
- [alacritty](https://github.com/Misterio77/dotfiles/blob/sway/home/.config/alacritty/alacritty.yml)
- [qutebrowser](https://github.com/Misterio77/dotfiles/blob/sway/home/.config/qutebrowser/config.py)
- [zathura](https://github.com/Misterio77/dotfiles/blob/sway/home/.config/zathura/zathurarc)
- [sway](https://github.com/Misterio77/dotfiles/blob/sway/home/.config/sway/config)
- [waybar](https://github.com/Misterio77/dotfiles/blob/sway/home/.config/waybar/colors.css)
- [rofi](https://github.com/Misterio77/dotfiles/blob/sway/home/.config/rofi/themes/colors.rasi)

Vóila. You're now ready to apply schemes.

#### Custom templates and schemes

To help manage your custom templates/schemes or your tweaks to pre-existing ones, flavours will also look in the user's `$XDG_CONFIG_HOME/flavours` directory, typically `~/.config/flavours`, when looking for templates/schemes. The folder structure should be the same as at `~/.local/share/flavours/base16/`.

Examples:
* Custom scheme `myscheme`: `$XDG_CONFIG_HOME/flavours/schemes/myscheme/myscheme.yaml`
* Custom template `mysoftware/mytemplate`: `$XDG_CONFIG_HOME/flavours/templates/mysoftware/templates/mytemplate.mustache`

Note, in case of conflict, schemes/templates in `$XDG_CONFIG_HOME/flavours` have priority over the ones in `${FLAVOURS_DATA_DIRECTORY:-~/.local/share/flavours}`.

#### Applying
`flavours apply` is the command you'll probably be using all the time, so it's built to be as useful as possible.

The syntax is `flavours apply [PATTERN]`, where PATTERN can be a scheme name, multiple scheme name, a glob (such as `*light`) expression, or can be ommited.
If more than one scheme is specified or matched, flavours will choose one randomly For example, `flavours apply *light` will pick one random scheme ending with "light", and apply it.

You can, for instance:
- Specify a scheme: `flavours apply pasque`
- Specify multiple schemes: `flavours apply pasque paraiso atlas`
- Use globs: `flavours apply "gruvbox*"`
- Omit the pattern: `flavours apply` (is the same as running `flavours apply "*"`)

#### Other commands
Other commands include:
- `flavours current` to see the last scheme you applied
- `flavours list [PATTERN]` to list all available schemes
- `flavours info [PATTERN]` to show info (including truecolor colored output) about some scheme(s)
- `flavours build <path_to_scheme> <path_to_template>` (see [Build](#Build) below)
- `flavours generate <dark|light> path/to/image/file` (see [Generate](#Generate) below)

#### Build
You can also use flavours as a simple [Base16 builder](https://github.com/chriskempson/base16/blob/master/builder.md). You can easily get a scheme path by using `flavours info theme_name | head -1 | cut -d '@' -f2`). This works great for automating static styles, and anything else you can come up with (I use it on my [personal website](https://misterio.me)).

#### Generate
Lastly, we have `flavours generate`, which can generate a scheme based on an image such as a wallpaper. By default, the scheme will be saved with the slug `generated`, but you can change it with `-s` or `--slug` or output to stdout instead with `--stdout`.

## Why
Why use this instead of other Base16 managers, or even pywal?

While these projects are great and certainly fit some people's workflow, they didn't quite fit mine.

I decided to do my own project because I wanted a simple CLI program that can easily integrate with rofi, polybar, and everything else in my workflow. The main feature is being able to quickly and easily use a curated list of schemes, on all your apps. It's also objectively faster than _any_ other manager.

## When
All features are implemented! I'm currently working on improving code quality and stabilizing the features, but everything should work as intended.

### Thanks to:
- Functionality inspiration from [Base16 Universal Manager](https://github.com/pinpox/base16-universal-manager)
- Logo inspiration from [Starship prompt](https://starship.rs)
