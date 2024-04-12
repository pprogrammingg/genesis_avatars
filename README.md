# Table of Contents
1. [Component Overview](#component-overview)
2. [Methods](#methods)
3. [mint_claim_nft_given_genav](#mint_claim_nft_given_genav)

# Component Overview
Genesis Avatar NFTs played in Realm256.

# Methods

## mint_claim_nft_given_genav

Given GENAV tokens, for each token:

1. Mint a claim NFT
2. Set claim NFT data to have:
   - `claimable_after_date`: equal to current instant
   - `sequence`: an incremental sequence number tracking the order of mint.
   
Returns claim NFT to the user.
