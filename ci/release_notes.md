## Features
- Add `/status` endpoint to get meta data about the state of the p2p module (connections / bootstrap state etc)
- Add `--force-seed=<hostname>:<port>` option to `daemon` command to force usage of a given seed

## Improvements

- Chnange flag `--use-tor=<bool>` to `--no-tor`. Passing the bool value is redundant.

## Bug Fixes

- Fix default `RISQ_HOME` dir (was missing `.risq` part)
- Add in previously unsupported currencies
