# iotasdk
An SDK for IOTA

##### Active Development Stage

## Build
`cargo build`

Use:

`iotasdk`

## install
### Nodes
1. bee [p4]
2. hornet [p4]
3. wasp [p3]
   1. --peer [p3]
      1. connect / trust [p3]
      2. disconnect / distrust [p3]
   2. --quorum [p3]

### Tools
4. schema [p1]
   1. init [p1]
   2. generate / build [p1]
      1. -wasm [p1]
         - --lang [p1]
           1. rust [p1]
           2. go [p1]

## --wasp (wasp-cli)
1. deploy chain [p2]

## Smart Contracts
1. Deploy [p2]
   1. -wasm [p2]
      - --lang [p2]
        1. rust [p2]
        2. go [p2]
   2. -evm [p3]
      - --lang [p3]
        1. solidity / sol [p3]
2. Interact / Transact [p2]
3. View [p2]
4. Listen / Webhook [p3]

---

Usage:

```sh
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
    schema      Schema Tool is used to generate Smart Contract Code. Try: `iotasdk schema
                --help`
```

