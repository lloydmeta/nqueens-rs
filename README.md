# NQueens [![Crates.io](https://img.shields.io/crates/v/nqueens.svg)](https://crates.io/crates/nqueens) [![Build Status](https://travis-ci.org/lloydmeta/nqueens-rs.svg?branch=master)](https://travis-ci.org/lloydmeta/nqueens-rs) [![nqueens](https://docs.rs/nqueens/badge.svg)](https://docs.rs/nqueens)

NQueens solver in Rust.

Highlights:

* Fairly fast
* Solver logic is pure and uses just higher-order functions

## Usage

`cargo install nqueens`

```shell
USAGE:
    nqueens --n <n>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -N, --n <n>    The number of queens and side length of the board you want to solve for
```