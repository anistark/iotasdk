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
    -v, --verbose            A level of verbosity, and can be used multiple times
    -V, --version            Print version information

SUBCOMMANDS:
    contract    Manage Smart Contracts
    help        Print this message or the help of the given subcommand(s)
    init        Initialise IOTASDK
    node        Configure and Control your IOTA Nodes
```

For node config:

```sh
Configure and Control your IOTA Nodes

USAGE:
    iotasdk node <NODESELECT> [OPTIONS]

ARGS:
    <NODESELECT>    Node Input. Available: bee, hornet, goshimmer, wasp

OPTIONS:
    -d                         Print debug info
    -h, --help                 Print help information
    -i, --install <INSTALL>    Install Node
    -p, --purge <PURGE>        Purge Node
    -r, --reset <RESET>        Reset Node
    -u, --update <UPDATE>      Update Node
        --upgrade <UPGRADE>    Upgrade Node
    -V, --version              Print version information
```


