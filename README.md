# cellrs

`cellrs` is a terminal-based battery indicator written in [Rust](https://www.rust-lang.org/).

---

## Platforms

Supported platforms/versions are generally based on [battery](https://crates.io/crates/battery).

- Linux 2.6.39+

## Install & Run

There are a few ways you can get and use `cellrs`.

- Download a release binary from [GitLab](https://gitlab.com/leglesslamb/cellrs/-/releases).
- Install from [crates.io](https://crates.io/crates/cellrs).

  ```sh
  cargo install cellrs
  cellrs
  ```

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

### Dependencies

- [battery](https://crates.io/crate/rust-battery)
- [chrono](https://crates.io/crate/chrono)
- [termion](https://crates.io/crates/termion)

---

## Acknowledgements

- [Valerio Besozzi](https://github.com/valebes)'s [rsClock](https://github.com/valebes/rsClock) for inspiring this project.
