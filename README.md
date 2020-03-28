# cellrs

![Classic Screenshot](screenshots/classic.jpg)

`cellrs` is a terminal-based battery indicator written in [Rust](https://www.rust-lang.org/).

## Overview

- `cellrs` displays a battery which resizes with the size of the terminal.
- The battery contains "cells" that scale to the current battery level. These cells are colour-coded based on the percentage of the battery.
- A status line is displayed below the battery showing numerical percentage and charging status.

### Platforms

Supported platforms/versions are generally based on [battery](https://crates.io/crates/battery).

- Linux 2.6.39+
- MacOS 10.10+
- Windows 7+
- FreeBSD
- DragonFlyBSD

### Prerequisites

- There no specific prerequisites to user `cellrs`.
- For developers, [Rust](https://www.rust-lang.org/), including [`cargo`](https://github.com/rust-lang/cargo/).

### Dependencies

- [battery](https://crates.io/crate/rust-battery)
- [chrono](https://crates.io/crate/chrono)
- [termion](https://crates.io/crates/termion)

## Install & Run

There are a few ways you can get and use `cellrs`.

- Download a release binary from [GitLab](https://gitlab.com/leglesslamb/cellrs/-/releases).
- Build from [source](https://gitlab.com/leglesslamb/cellrs).

  ```sh
  git clone https://gitlab.com/leglesslamb/cellrs.git
  cd cellrs
  cargo build --release
  ./target/release/cellrs
  ```

---

## Development

- **GitLab Repo**: [gitlab.com/leglesslamb/cellrs](https://gitlab.com/leglesslamb/cellrs).
- **Crate Listing**: [crates.io/crate/cellrs](https://crates.io/crates/cellrs).

---

## Acknowledgements

- [Valerio Besozzi](https://github.com/valebes)'s [rsClock](https://github.com/valebes/rsClock) for inspiring this project.
