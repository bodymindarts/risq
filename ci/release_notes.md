## Features

- removed `/offers` api endpoint and added `offers` query to the [graphql schema](https://github.com/bodymindarts/risq/blob/master/src/api/schema.graphql)
- updated cli to hit the graphql endpoint and add `--market CUR` arg to the `offers` sub command (to filter offers by non BTC currency)

## Improvements
- incoming messages that Add / Refresh offers are now cryptographically verified
- some scenarious where threads would crash have been fixed
