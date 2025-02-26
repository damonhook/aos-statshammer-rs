# AOS Statshammer (Rust)

> [!NOTE]
> This is a work in progress as it is being reworked for v4 of AoS. 
> To view a more complete Rust REST API based on v3, see [here](https://github.com/damonhook/aos-statshammer-rs/tree/0cc6ffa5d0651dd7494f1da0bc79a8355b501014)

This is a re-imagining of the AOS Statshammer project written in Rust.

> [!NOTE]
> If you are looking for the AOS Statshammer web app, head to:
> - Website: https://aos-statshammer.damonhook.com
> - Github: https://github.com/damonhook/aos-statshammer

## Crates

- `aos-statshammer-core`: This contains all the core logic for the application
- `aos-statshammer-cli`: This contains a CLI tool for calculating the average damage for AOS weapons

## Using The CLI

TLDR

```bash
cargo run -p aos-statshammer-cli -- -w <WEAPON TOML FILE> -t <TARGET TOML FILE>
```

There are a few example files included in `aos-statshammer-cli/examples`. E.g:

```bash
cargo run -p aos-statshammer-cli -- -w ./aos-statshammer-cli/examples/weapons/simple.toml -t ./aos-statshammer-cli/examples/targets/simple.toml
```