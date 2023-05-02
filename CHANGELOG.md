# Changelog

## [0.7.1](https://github.com/Misterio77/flavours/releases/tag/v0.7.1)

### Fixes

- Add missing newline when applying ([#78](https://github.com/misterio77/flavours/issues/78))
- Set minimum supported rust version ([#44](https://github.com/misterio77/flavours/issues/44))

## [0.7.0](https://github.com/Misterio77/flavours/releases/tag/v0.7.0)

### Additions

- Added a `-t`/`--template` flag for `flavours list`, to list templates. It supports all the usual `list` options (single line, globbing), and even works for subtemplates if you add a `/` (e.g. `flavours list '*/default'`) ([#67](https://github.com/Misterio77/flavours/pull/67)). Thanks, [@probably-neb](https://github.com/probably-neb)!
- You can now use `{scheme}` as a `subtemplate`, this is useful when you have additional subtemplates to fix some issues specific schemes might have, it falls back to `default` if one is not found ([#66](https://github.com/Misterio77/flavours/pull/66)). Thanks, [@dive-deeper](https://github.com/dive-deeper)!

### Fixes

- Document Homebrew package ([#74](https://github.com/Misterio77/flavours/pull/74)). Thanks, [@awanwar](https://github.com/awanwar)!
- Performance improvements ([#71](https://github.com/Misterio77/flavours/pull/71)). Thanks, [@vrmiguel](https://github.com/vrmiguel)!
- We now use the awesome [base16_color_scheme](https://lib.rs/crates/base16_color_scheme) crate as our base16 primitives ([#76](https://github.com/Misterio77/flavours/pull/76)). Thanks, [@titaniumtraveler](https://github.com/titaniumtraveler)!

## [0.6.0](https://github.com/Misterio77/flavours/releases/tag/v0.6.0)

### Additions

- Flavours will now look for schemes and templates in your configuration directory as well ([#48](https://github.com/Misterio77/flavours/pull/48)). Thanks, [@loiccoyle](https://github.com/loiccoyle)!
- You can now add `[[extra_scheme]]` and `[[extra_template]]` entries to your configuration to configure sources declaratively [#62](https://github.com/Misterio77/flavours/pull/62). Thanks, [@Softsun2](https://github.com/Softsun2)!

## [0.5.1](https://github.com/Misterio77/flavours/releases/tag/v0.5.1)

### Changes

- Renamed `item` to `items` on the config. Backwards compatibility is maintained.

### Additions

- Adds support for [base24](https://github.com/tinted-theming/base24) schemes and templates.

### Fixes

- Workaround an issue with `cargo install` not respecting locked versions.

## [0.5.0](https://github.com/Misterio77/flavours/releases/tag/v0.5.0)

### Additions

- Added a `build` subcommand, which works as traditional base16 builder. Useful for automation.
- The `apply` command can now accept schemes from stdin using the `--stdin` flag.
    - Useful for chaining with `generate`: `flavours generate dark --stdout example.png | flavours apply --stdin`

## [0.4.0](https://github.com/Misterio77/flavours/releases/tag/v0.4.0)

### Additions

- It is now possible to specify which shell to run hooks on. Allowing for bash syntax (or any other shell) syntax.
    - It defaults to `sh -c '{}'`, which should work on any POSIX system.

## [0.3.6](https://github.com/Misterio77/flavours/releases/tag/v0.3.6)

### Additions

- Added `light` configuration for apply items, this lets you specify which hooks should be skipped when running `apply` with the new `--light` flag.

## [0.3.5](https://github.com/Misterio77/flavours/releases/tag/v0.3.5)

### Fixes

- Fixed an issue with the alpha channel when generating colors ([#22](https://github.com/Misterio77/flavours/pull/22)). Thanks, [@loiccoyle](https://github.com/loiccoyle)!

## [0.3.4](https://github.com/Misterio77/flavours/releases/tag/v0.3.4)

### Fixes

- BrokenPipe panic when piping flavours info to something that closes the stream during execution ([#16](https://github.com/Misterio77/flavours/issues/16))
- Decrease minimum luma for dark backgrounds, to avoid ignoring good, saturated-enough colors


## [0.3.3](https://github.com/Misterio77/flavours/releases/tag/v0.3.3)

### Fixes

- Improves background in light schemes and accent on dark schemes
  - Increased minimum luma and decrease maximum saturation of light scheme backgrounds colors.
  - Increased minimum luma of dark scheme accent colors.


## [0.3.2](https://github.com/Misterio77/flavours/releases/tag/v0.3.2)

### Changes

- Printing colored output with `info` is now the default behavior.
  - You can use `-r`/`--raw` to avoid it.

### Additions

- Add the `generate` sub-command
  - You can generate schemes based on an image, choosing whether you want a `dark` or `light` scheme
  - Example usage: `flavours generate dark wallpaper.png && flavours apply generated`


## [0.2.2](https://github.com/Misterio77/flavours/releases/tag/v0.2.2)

### Additions

- Added proper scheme completions for fish

## [0.2.0](https://github.com/Misterio77/flavours/releases/tag/v0.2.0)

### Additions

- Added `info` sub-command
  - Allows you to see any scheme info in a human-readable way.
  - It shows name, author, colors (in hex), and shows the actual schemes colors if your terminal supports it.


## [0.1.5](https://github.com/Misterio77/flavours/releases/tag/v0.1.5)

### Changes

- Improve how we store the templates, sources and lists repositories.
  - Instead of a full clone, we now just do a shallow one, checkout exactly what we need and delete the .git

## [0.1.4](https://github.com/Misterio77/flavours/releases/tag/v0.1.4)

### Fixed

- Update to latest clap version.

## [Beta 0.1.2](https://github.com/Misterio77/flavours/releases/tag/v0.1.2)

### Fixed

- Create default config if it doesn't exist. Better handling of non-existent directory.
- Removed clap logic from module files
- Downgraded clap version to crates.io version (non-git)
- Added `Cargo.lock`
- Published on crates.io


## [Beta 0.1](https://github.com/Misterio77/flavours/releases/tag/v0.1)

Initial release
