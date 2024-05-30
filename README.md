# Table of Contents


# Component Overview
Genesis Avatar NFTs played in Realm256.


# TestNet 
- Generate Manifests

# Methods

## mint_claim_nft_given_genav

Given GENAV tokens, for each token:

1. Mint a claim NFT
2. Set claim NFT data to have:
   - `claimable_after_date`: equal to current instant
   - `sequence`: an incremental sequence number tracking the order of mint.
   
Returns claim NFT to the user.


# Package Deployment

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


# Troubleshooting
- Some times one can run some tests for example in sitatuin where transaction manifest run from the console shows some hexadecimal string being in the error. if it is NodeId(some hex text), most likely can be converted to a bech32 address (those that start with `resource_`, `component_` etc)

see commented out tests for sample conversion. Also, run test with 

```
 cargo test -- --nocapture
```

to be able ot use `println!`.

# TODO

- Use array to track what ipfs value is used []

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



