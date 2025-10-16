# sats-converter

![Weekend Project](https://img.shields.io/badge/kind-weekend_project-blue)

Sats are the lowest denomination of bitcoin, and the one 
you use in everyday transactions. One bitcoin corresponds to 
100.000.000 sats.

I wanted to quickly make conversions between euros and sats,
and to build a CLI tool in Rust after not using the language for
a while, so I took a couple of hours to write this tool.

It fetches the bitcoin to euro price from the Kraken API, 
and performs the conversion locally.

## Installation

If you have Rust installed, you can install this tool
in your machine by running the following command in the repo directory:

```sh
cargo install --path .
```

## Usage

To convert 1000 sats to euros:

```sh
sats-converter 1000
```

To convert 1000 euros to sats:

```sh
sats-converter -c eur 1000
```






