# Decentralized voting system

This chain will allow users to create, vote on, and manage proposals on the blockchain.

## Create Proposals:

Users can create new proposals by submitting a description and a duration for the voting period.

Proposals are stored on-chain with a unique identifier, the creator's address, the description, and the voting period.

## Vote on Proposals:

Users can cast votes on active proposals.

Each user can only vote once per proposal.

Votes can be either "Yes" or "No".

## End Voting Period:

Once the voting period for a proposal ends, the results are finalized.

The proposal is marked as approved if the majority of votes are "Yes," otherwise, it is rejected.

## Getting Started

Depending on your operating system and Rust version, there might be additional
packages required to compile this template. Check the
[Install](https://docs.substrate.io/install/) instructions for your platform for
the most common dependencies. Alternatively, you can use one of the [alternative
installation](#alternatives-installations) options.

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```


### Single-Node Development Chain

The following command starts a single-node development chain that doesn't
persist state:

```sh
./target/release/polkadot-dectralized-voing-node --dev
```

To purge the development chain's state, run the following command:

```sh
./target/release/polkadot-dectralized-voing-node purge-chain --dev
```

To start the development chain with detailed logging, run the following command:

```sh
RUST_BACKTRACE=1 ./target/release/polkadot-dectralized-voing-node -ldebug --dev
```

Development chains:

- Maintain state in a `tmp` folder while the node is running.
- Use the **Alice** and **Bob** accounts as default validator authorities.
- Use the **Alice** account as the default `sudo` account.
- Are preconfigured with a genesis state (`/node/src/chain_spec.rs`) that
  includes several prefunded development accounts.


### Connect with Polkadot-JS Apps Front-End

After you start the node locally, you can interact with it using the
hosted version of the [Polkadot/Substrate
Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944)
front-end by connecting to the local node endpoint. A hosted version is also
available on [IPFS (redirect) here](https://dotapps.io/) or [IPNS (direct)
here](ipns://dotapps.io/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).
