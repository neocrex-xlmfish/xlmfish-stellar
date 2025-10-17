# xlmfish-stellar

This repository contains Rust scripts for working with Stellar. 

Currently, it includes a simple configuration loader that reads the network environment and Horizon URL from `Config.toml`.

This is a work in progress and will be updated...

---

```
xlmfish-stellar/
├── Config.toml
├── assets/
│   ├── stellar/
│   │   ├── testnet.toml
│   │   └── public.toml
│   └── xrp/
│       ├── testnet.toml
│       └── mainnet.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── blockchain.rs
│   ├── stellar_chain.rs
│   └── types.rs
```