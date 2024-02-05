# Pkg

Pkg is a command line front-end for package management in Fedora Linux written in Rust.

## Features

- Dnf (.rpm)

- Flatpak

- COPR (Cool other package repo)

## Install

You will need to build Pkg from source using cargo.

**Dependencies**

```
$ sudo dnf install git rustup
```

```
$ rustup-init
```

**Building**

```
$ git clone https://github.com/Empyrean-Void/Pkg ~/code/rust --depth 1
```

```
$ cd ~/code/rust/pkg/
```

```
$ cargo build --release && cp target/release/pkg ~/.cargo/bin
```
