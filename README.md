# goku

![goku](https://i.pinimg.com/originals/3e/4f/25/3e4f25362a84f7499ecd607b6ecc1183.jpg)

## Overview
[![Actions Status](https://github.com/k-nasa/goku/workflows/CI/badge.svg)](https://github.com/k-nasa/goku/actions)


goku is a HTTP load testing application written in Rust

(This is inspired by [vegeta](https://github.com/tsenart/vegeta)!)

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

## Licence

[MIT](https://github.com/k-nasa/goku/blob/master/LICENCE)

## Author

[k-nasa](https://github.com/k-nasa)

[my website](https://k-nasa.me)
