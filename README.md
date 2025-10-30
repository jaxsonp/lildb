
# LilDB

## Crate/target structure

- `/api`: Defines the server-client communication protocol and data encoding/decoding implementation
- `/cli`: A CLI client, implements the client library
- `/client`: A library encapsulating interaction with a server
- `/query-lang`: LQL (LilDB Query Lang) parser library
- `/server`: Guess

## Database management notes

- database names are case-insensitive

## Server

### Configuration

Config file path may be supplied either through the `LILDB_CONFIG_PATH` environment variable, or by passing it as the first argument into the binary (which takes precedence). The config file should follow the following INI-like format:

```
# comment
KEY1=value

# another comment
[section]
KEY2=another value
KEY3=yet another value
```

Below are all the configurable options:

| Name | Description | Default |
|---|---|---|
| `DATA_PATH` | Path to directory where lildb data is stored | `./lildb-data` |
| `LISTEN_ADDR` | Address to receive connect requests on | `*` |
| `LISTEN_PORT` | Port to receive connect requests on | `11108` |

## TODO

- Polish config reading (INI-ize)
- Work on query lang
	- Tests
- Improve logging
- Improve data security/permission checking