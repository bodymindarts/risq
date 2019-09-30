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

## Discussion

discussion of the individual points pending

## Setup

You need to install [rust](https://www.rust-lang.org/tools/install) and [tor](https://people.torproject.org/~sysrqb/webwml/docs/installguide.html.en)

Eg for mac:
```
curl https://sh.rustup.rs -sSf | sh
brew install tor
```

Then use the [make](./Makefile) commands for building / testing / running
Eg:
```
make run-tor
make run
```
