# SP1 Tendermint Template

An example of a Tendermint light client on Ethereum powered by SP1.

> [!CAUTION]
>
> This repository is still an active work-in-progress and is not audited or meant for production usage.


## Overview

The SP1 Tendermint template is a simple example of a ZK Tendermint light client on Ethereum powered by SP1. It demonstrates how to use SP1 to generate a proof of the update between two Tendermint headers and verify it on Ethereum.

* The `program` directory contains a Succinct zkVM program that implements Tendermint light client verification logic.
* The `prover` generates a consensus proof from one height to another

## Run Tendermint Light Client End to End

* Follow instructions to install [SP1](https://succinctlabs.github.io/sp1/).
* Install [Forge](https://book.getfoundry.sh/getting-started/installation.html).


1. Export your SP1 Prover Network configuration
    ```shell
    # Export the PRIVATE_KEY you will use to deploy the contract & relay proofs.
    export PRIVATE_KEY=<PRIVATE_KEY>

    # To use the Succinct proving network, set `SP1_PRIVATE_KEY` to your private key on the proving network.
    export SP1_PRIVATE_KEY=<SP1_PRIVATE_KEY>
    ```

2. Run the Tendermint operator.
    ```shell
    cargo run --bin operator --release
    ```