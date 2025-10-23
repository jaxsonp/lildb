
# LilDB daemon

## Config file

Config file path must be supplied, either through the `LILDB_CONFIG_PATH` environment variable, or by passing it as the first argument into the binary, which takes precedence. The config file should follow the following format:

```
# comment
KEY1=value

# another comment
KEY2=another value
KEY3=yet another value
```

Below are all the configurable options:

| Name | Description | Default |
|---|---|---|
| `DATA_PATH` | Path to directory where lildb data is stored | `./lildb-data` |
| `LISTEN_ADDR` | Address to receive connect requests on | `*` |
| `LISTEN_PORT` | Port to receive connect requests on | `11108` |