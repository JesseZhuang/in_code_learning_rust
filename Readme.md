# Rust Installation

Rust recommends using `rustup`, which can manage multiple versions of rust.

## Mac

1. `brew install rustup`
2. `/usr/local/Cellar/rustup-init/1.25.1/bin/rustup-init`
3. following instructions to continue
4. verify `rustc` version

```shell
$ rustc --version
rustc 1.66.0 (69f9c33d7 2022-12-12)
```

Then follow https://plugins.jetbrains.com/plugin/8182-rust/docs/rust-quick-start.html for IntelliJ Rust configuration.

## IntelliJ Project/Workspace Setup

File -> Project Structure -> Modules -> + -> import module -> choose rust package folder (under crates/)
