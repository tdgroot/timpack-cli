# timpack-cli
A command-line-interface to make my life easier.

## How to build
To build the application, you need rust and cargo installed.

### Fedora dependencies
`sudo dnf install rust cargo`

### Building the application

```
cd /path/to/timpack-cli-project
cargo build
cargo run php72 cc list
```

## Installation
The cli is only installable with Rust's [Cargo](https://crates.io/).

```
cargo install timpack-cli
```

Make sure you have the cargo bin directory in your `PATH`.

```
export PATH=$PATH:$HOME/.cargo/bin
```

## How to use
```
# Run composer update with default php interpreter
timpack-cli cc update

# Run composer update with php 7.1
timpack-cli php71 cc update

# Run n98-magerun cache:flush with php 7.0
timpack-cli php70 m1 cache:flush

# Run n98-magerun2 cache:flush with php 7.1
timpack-cli php71 m2 setup:upgrade
```

There are some aliases:
- cc => composer
- m1 => n98-magerun
- m2 => n98-magerun2
