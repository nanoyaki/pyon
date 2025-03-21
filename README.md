# Pyon

Print ASCII and braille bunnies to your terminal!

I forked the cute meow tool from PixelSergey and made it bunnies instead.

## Usage

```
Usage: pyon [OPTIONS]

Options:
  -c, --count <COUNT>  How many bunnies to print [default: 1]
  -l, --literally      Are you literally this bunny?
  -h, --help           Print help
  -V, --version        Print version
```

## Installation

### From Cargo

```sh
cargo install --git https://github.com/nanoyaki/pyon
```

The binary will then be built to some directory that will be output to your command line.

## Building from source

1. Install Rust
1. Clone this repository
1. Build and run with `cargo run` or `cargo run -- [OPTIONS]`
