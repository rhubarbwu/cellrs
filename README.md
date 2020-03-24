# cellrs

`cellrs` is a terminal-based battery indicator written in [Rust](https://www.rust-lang.org/).

## Dependencies

These are the external crate dependencies used.

- [battery](https://github.com/svartalf/rust-battery).
- [chrono](https://github.com/chronotope/chrono).
- [termion](https://gitlab.redox-os.org/redox-os/termion).

## Supported platforms

Supported platforms are based on the supported platforms of [battery](https://github.com/svartalf/rust-battery).

- Linux 2.6.39+
- MacOS 10.10+
- Windows 7+
- FreeBSD
- DragonFlyBSD

## Install & Build

```sh
git clone https://gitlab.com/leglesslamb/cellrs.git
cd cellrs
cargo build --release
./target/release/cellrs
```

## Usage

```sh
usage : cellrs
```
