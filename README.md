# goku

![goku](https://user-images.githubusercontent.com/23740172/68545732-1ae00480-0413-11ea-8db9-3ceaafdb3b91.jpg)

## Overview
[![Actions Status](https://github.com/k-nasa/goku/workflows/CI/badge.svg)](https://github.com/k-nasa/goku/actions)
[![crate-name at crates.io](https://img.shields.io/crates/v/goku.svg)](https://crates.io/crates/goku)


goku is a HTTP load testing application written in Rust

(This is inspired by [vegeta](https://github.com/tsenart/vegeta)!)

## Demo

![goku](https://user-images.githubusercontent.com/23740172/68545671-92616400-0412-11ea-86f3-dba3a80f2227.gif)


## Installation

#### using cargo

```console
cargo install goku
```

## Usage

```console
goku 0.1.0
goku is a HTTP load testing application written in Rust

USAGE:
    goku [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help          Show help
    kamehameha    Run load test [aliases: attack]
```

## Example

### kamehameha
Send 10,000 requests to 127.0.0.1:8080 in 10 parallel
(There is an attack alias because hitting 'kamehameka' is awkward.)

```console
goku kamehameha -c 10 -n 10000 'http://127.0.0.1:8080'
# or goku attack -c 10 -n 10000 'http://127.0.0.1:8080'
```

Output in json and text is possible. When combined with jq, the display can be made beautiful.

```console
goku kamehameha -c 10 -n 10000 'http://127.0.0.1:8080' -o json | jq .

{
  "errors": [],
  "concurrency_level": 10,
  "time_taken_test": {
    "secs": 2,
    "nanos": 209216142
  },
  "complete_requests": 10000,
  "failed_requests": 0,
  "total_transferred": 1290000,
  "total_time": {
    "secs": 22,
    .
    .
    .
```

## Contribution

1. Fork it ( http://github.com/k-nasa/goku )
2. Create your feature branch (git checkout -b my-new-feature)
3. Commit your changes (git commit -am 'Add some feature')
4. Push to the branch (git push origin my-new-feature)
5. Create new Pull Request

## License

[MIT](https://github.com/k-nasa/goku/blob/master/LICENSE)

## Author

[k-nasa](https://github.com/k-nasa)

[my website](https://k-nasa.me)
