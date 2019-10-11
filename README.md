# Risq - feasibility study on re-implementing bisq in rust

[Bisq](https://github.com/bisq-network/bisq) is a decentralized application that allows trading of Bitcoin and other digital assets via a peer-to-peer trading protocol.
Currently a [proposal is being discussed](https://github.com/bisq-network/proposals/issues/32) regarding a backwards incompatible upgrade to the trading protocol that would rely on trading partners using the BSQ coloured coin as collateral when trading.
This and other improvements are being proposed to launch as [Bisq v2](https://github.com/bisq-network/proposals/issues/118).
As this would likely require re-writing large parts of the application, the question has been put forward wether it might be worth starting from scratch rather than take on the legecy of the existing code base.

This repo represents a spike to investigate the feasibility and effort required to rewrite the parts of the app needed for interop with V1

## Goals

to shed some light on the following questions
- is an alternative implementation that is compatible with the live p2p network possible (ie. can java <-> rust processes communicate correctly via the protobuf based protocol)?
- are there any significant technical advantages that can be gained from taking this approach (eg. less overall complexity, less risky dependencies, better dev workflow etc.)?
- how high would the remaining effort be to achieve production rediness with an alternative implementation?
- does it make sense as a strategic approach to write V2 from scratch vs adapt the existing code?

## Setup

You need to install [rust](https://www.rust-lang.org/tools/install) and [tor](https://people.torproject.org/~sysrqb/webwml/docs/installguide.html.en)

Eg for mac:
```
$ curl https://sh.rustup.rs -sSf | sh
$ brew install tor
```

Then use the [make](./Makefile) commands for building / testing / running
Eg:
```
$ make build
```

## Demo

Once the project has been built with `make build` a binary will be under `./target/debug/risq`

```
$ ./target/debug/risq help
USAGE:
risq [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    daemon    Runs the risq p2p node [aliases: d]
    help      Prints this message or the help of the given subcommand(s)
    offers    Subcomand to interact with offers
```

You can see help for the individual subcommands via:
```
$ ./target/debug/risq help daemon
<omitted>
```

Run the daemon after starting tor:
```
$ make run-tor
$ RUST_LOG=debug ./target/debug/risq d
```

It will take a while to bootstrap the data from the seed node (currently no data is persisted so bootstrap must execute every time you start the daemon).

From a different console you can check that the api is running via:
```
$ curl localhost:7477/ping
pong
$ curl -s localhost:7477/offers | jq
{
  "offers": []
}
```

Or use the cli to print the offers (once its bootstraped)
```
$ ./target/debug/risq offers
OPEN OFFERS
Sell fixed 5000 25000000(10000000)
Buy fixed 303000000 1000000(1000000)
Buy market 0.0099 1000000(1000000)
Buy market -0.0111 1110000(1110000)
```

## Limitations

As this is a proof of concept there are a number of limitations.
- No data is persisted so bootstrap is required for each run.
- Only 1 connection to a seed node is currently established. If the initial data sync fails then daemon will not bootstrap properly or be able to join the p2p network.
- Currently only 1 read-only endpoint is provided (offers).
- Not much effort has been made to make the output look pretty or be perticularly usefull other than seeing that things are alive.

## Node Checker

To build and run the node checker do the following:
```
make build-with-checker
./target/debug/risq check-node 5quyxpxheyvzmb2d.onion 8000
```

it has the following exit codes:
- 0 - Ping - Pong succeeded
- 1 - Connection couldn't be established
- 2 - Ping couldn't be sent
- 3 - Issue while waiting for Pong
```
