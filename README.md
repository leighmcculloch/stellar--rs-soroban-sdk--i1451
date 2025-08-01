This repository contains an example of using the change proposed and being experimented with in:
- [stellar/rs-soroban-sdk#1507]

It contains two Rust crates:

- [`contract`] that contains a Stellar contract that implements the library's `Pause` trait, relying on the traits default implementation.
- [`library`] that contains the `Pause` trait and its default implementation

[stellar/rs-soroban-sdk#1507]: https://github.com/stellar/rs-soroban-sdk/pull/1507
[`contract`]: ./contract/src/lib.rs
[`library`]: ./contract/src/lib.rs

## Usage

```
cd contract
stellar contract build
```

## Feedback

Please provide feedback on the pull request above.
