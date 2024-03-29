# iotasdk
An SDK for IOTA

##### Active Development Stage
Built using [clap](https://github.com/clap-rs/clap).
## Build
`cargo build`

---

Usage:

```sh
  _           _                       _   _         ___         ___        _
 (_)   ___   | |_    __ _   ___    __| | | | __    / _ \       / _ \      / |
 | |  / _ \  | __|  / _` | / __|  / _` | | |/ /   | | | |     | | | |     | |
 | | | (_) | | |_  | (_| | \__ \ | (_| | |   <    | |_| |  _  | |_| |  _  | |
 |_|  \___/   \__|  \__,_| |___/  \__,_| |_|\_\    \___/  (_)  \___/  (_) |_|


iotasdk 0.0.1

Kumar Anirudha <mail@anirudha.dev>

IOTASDK :: A Swiss Knife for all things IOTA

USAGE:
    iotasdk [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --config <CONFIG>    Sets a custom config file. Could have been an Option<T> with no default
                             too [default: default.conf]
    -h, --help               Print help information
    -V, --version            Print version information

SUBCOMMANDS:
    contract    Deploy and Interact with Smart Contracts with VM specific and Chain Specific
                configs. Try: `iotasdk contract --help`
    help        Print this message or the help of the given subcommand(s)
    info        Get all info. Try: `iotasdk info --help`
    init        Initialise IOTASDK. Try: `iotasdk init --help`
    node        Setup and Configure IOTA Nodes. Try: `iotasdk node --help`
    schema      Schema Tool is used to generate Smart Contract Code. Try: `iotasdk schema --help`
```

