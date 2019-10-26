# [risq release v0.2.0](https://github.com/bodymindarts/risq/releases/tag/v0.2.0)

## Features
- add `ticker` `volumes` and `depth` query. Now feature complete with bisq markets api.

# [risq release v0.1.1](https://github.com/bodymindarts/risq/releases/tag/v0.1.1)

## Improvements

- test build for arm-unknown-linux-gnueabihf target (eg. raspi) and attach binary to release

# [risq release v0.1.0](https://github.com/bodymindarts/risq/releases/tag/v0.1.0)

## Features
- add hloc query to graphql

## Improvements
- better responsiveness of api during bootstrapping via bulk insert of trade history
- test that there are no warnings when building without any features in ci pipeline
- automatically commit version as <version>-dev in Cargo.{toml,lock} after a version bump

# [risq release v0.0.7](https://github.com/bodymindarts/risq/releases/tag/v0.0.7)

## Improvements
- Logging only for the `daemon` command. Default log level is `info`, this can be changed using the new flag `--log-level` ([#12](https://github.com/bodymindarts/risq/pull/12))

## Bug fixes
- Fixed timestamp bug in price nodes ([#15](https://github.com/bodymindarts/risq/pull/15))

# [risq release v0.0.6](https://github.com/bodymindarts/risq/releases/tag/v0.0.6)

## Features

- removed `/offers` api endpoint and added `offers` query to the [graphql schema](https://github.com/bodymindarts/risq/blob/master/src/api/schema.graphql)
- updated cli to hit the graphql endpoint and add `--market CUR` arg to the `offers` sub command (to filter offers by non BTC currency)

## Improvements
- incoming messages that Add / Refresh offers are now cryptographically verified
- some scenarious where threads would crash have been fixed

## Bug fixes
- fix issue with the ci pipeline release process
