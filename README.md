# Table of Contents

# Component Overview
Genesis Avatar NFTs played in Realm256.


# Typical Testing of the conversion of Genav token to GAT (Genesis Avatar Token)
Note: use manifest files under `transaction-manifest/manifests` folder
1. Setup: if testing on testnet, run `mint_tgenav.rtm` with appropriate params to get test GENAV tokens
2. Run `mint_claim_nft_given_genav.rtm` to recieve a claim NFT given test GENAV tokens as input
3. Run `mint_gat_given_claim_nft.rtm` to get GAT tokens


# Package Deployment (WIP)

- Update config via run script.

Pre-requisites: Python virtual evn is needed to run python scripts. Use terminal in the root folder run:

```
    python3 -m venv .venv
``` 
and 

```
    source .venv/bin/activate
```

Run

```
    which python
```

To make sure you are using virtual env.



# Dev tips and Troubleshooting 
- Some times one can run some tests for example in sitatuin where transaction manifest run from the console shows some hexadecimal string being in the error. if it is NodeId(some hex text), most likely can be converted to a bech32 address (those that start with `resource_`, `component_` etc)

see commented out tests for sample conversion. Also, run test with 

```
 cargo test -- --nocapture
```

to be able ot use `println!`.

# TODO

- Use array to track what token_id value is used []
- [bug] One extra claim is minted , e.g. 2 GENAV input gives 3 [fixed]
- make function that receives Claim NFT and mints GAT [x]
- Python env to run script locally, docs [x]
- instantiate and fill the rest of config [x]
- generate all manifest (for now leave change metadata out) [x]
- docs for instantiation and deployment 
    - manifest gen section include `python3 -m venv .venv` and `source .venv/bin/activate`
- make priliminary test cases [x]
- manifest for change metadata
- test cases for metadata change



