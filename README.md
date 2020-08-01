# huobi-chain contract template

This is an example and template of huobi-chain contract written in Rust.

You can refer <https://huobigroup.github.io/huobi-chain-docs/#/contract_demo> to learn how to write
a contract in C language.

## Usage

Steps to use this template:

1. Install Docker.
2. Clone this repo.
3. Edit the contract and test cases as you wish.
4. Use commands in `Makefile` to compile the contract and test.

The `contract` directory is the contract rust project.
It should be a **no-std** rust binary project.
You can edit the `contract/src/main.rs` to write your own logic.

the `src` directory contains the contract test.
The main logic is in `src/contract.rs`.
You can edit the file to add your own test cases.

```
# `make` will format the code, build the debug version binary and run the test.
# The contract file will be at `contract/build/debug/contract`.
$ make

# The contract file will be at `contract/build/release/contract`.
$ make build-contract-release

```