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

1. Clone repo [](https://github.com/combased-dev/jewel-radix-liquid-stake/)
1. Change to [](https://github.com/combased-dev/jewel-radix-liquid-stake/tree/main/jewel-xrd-defi) directory where `cargo.toml`is.
2. **Delete Cached Artifacts**: If there is a target folder,make sure there are no cached. For this, delete `./target/wasm32-unknown-unknown/release/jewel_xrd_defi.wasm` and `./target/wasm32-unknown-unknown/release/jewel_xrd_defi.rpd`. 
3. Execute `scrypto build`, new `.wasm` and `.rpd` should be created.
4. **Developer Console**: Navigate to developer console, either [Stokenet](https://stokenet-console.radixdlt.com/) or [Mainnet](https://console.radixdlt.com/).
5. **Connect Wallet**: from top right corner, click `Connect`. Inside wallet choose perona and account. 
6. **Prepare to Deploy**: From the left navigation menu, click `Deploy Package`. Here,
upload `.wasm` and `.rpd` files created in the previous steps. A deploy package badge can be either created on the fly or an existing one can be used. This `package deployment badge`(essentially a funginle or non-fungible) should not to be confused with `dApp ownder badge` which is a badge we use to give our dApp an owner.
7. **Deploy Package**: Once the `.wasm` and `.rpd` files as well as `package deployment badge` are selected, press `submit`.**NOTE**: Assuming that Radix standard wallet is used, make sure a fee payer is selected, if not click `customize`, then `Select Fee Payer` and then select an account to pay the trnasaction fee. **Note**  if testing on `Stokenet` give any account some free XRD by clicking the 3 dots (`...`) on top of account screen, then `Dev Preferences` and finally `Get XRD Test Tokens`.
8. If success, paste the created package address under `config/global_config.json` key `package_info.package_address`.This value is important for the instantiation step.


# dApp Instantiation

1. Choose a new Radix wallet account or an existing account to `dApp owner`. Write this account address under `config/global_config.json` under key `dapp_accounts.dapp_owner_account_address`.

2. **dApp Owner Badge**: Create a dApp owner badge via transaction manifest file `transaction-manifest\templates\create_dapp_owner_badge.rtm`. To do this, copy the content of the file, navigate to Radix devleoper console (links shared previously) and click `Send Raw Transaction`. Paste the file content and modify `Address("<dapp_owner_account_address>")` to replace the angle brackets and placeholder name with thedapp_owner_account_address from the previous step. Submit the transction.

3. Radix walelt should pop up a screen indicating intent to deposit this dApp Owner Badge with name `Jewel Liquid Staking Owner Badge` and symbol `JEWEL_OWNER_BADGE`.

4. When a transaction on wallet successfully, desktop browser will have a transaction link, click that link to go to Radix explorer. Under `Transaction`, make sure `Summary` is selected (the other options are `Details` and `Raw receipts`). 

Towards the bottom find `Account Balance Changes` and click on 
`Jewel Liquid Staking Owner Badge`. Once resource details open, record the created dApp owner badge address in `config/global_config.json` under `jewel_dapp_owner_badge_address`  key at the root of the config file.

5. **Create Automatic Job Account**: Create an an automatic job account which will be used by the four automatic jobs.

```bash
    cd utils/key_gen
    ./run.sh
```

Copy and paste the generated account in to `config/global_config.json`
under key `dapp_accounts.auto_job_account_address`.

**Note**: see [Security Considerations](#security-considerations) for recommendation on using separate automatic jobs for each task rather than one.

6. **User accounts for testing**: Enter account numbers for user a and b for testing. These ideally should be accounts under the tester's control in their Radix wallet. Put the account addresses under `config/global_config.json`
under key `dapp_accounts.user_a_account_address` and `dapp_accounts.user_b_account_address` as appropriate.

7. **dApp Instantiation** : Copy the contents of manifest file`transaction-manifest\templates\instantiate_jewel_defi.rtm`. Paste this under `Send Raw Transaction` page under developer console. Replace  `Address("<package_address>"), Address("<jewel_dapp_owner_badge_address>")` and `Address("<dapp_owner_account_address>")` with the appropriate values from the config. All of these values should be have been recorded manually in the config file from the previous steps.

8. Submit the transaction. Radix wallet should pop up confirmation intent to deposit 4 extra badges as output of instantiation. These are badges to be used by each of 4 automatic jobs. The dApp is now instantiated and the component is globabalized meaning that we can interact with it on the DLT.

9. In developer console click the successful transaction link. Copy the transaction Id, this will be used for the next step.

10. Now, we can generate the rest of `config/global_config.json` based on the `instantiate_transaction_id`. Change directory to `transaction-manifest`, then run

```bash
    # replace <instantiate_transaction_id> with the correct value
    ./run.sh <instantiate_transaction_id>
```

The script queries Radix DLT based on `<instantiate_transaction_id>`, extracts created enteties and populates values under `config/global_config.json` under key `dapp_instant_info`. 

Additionally, the script will copy the files under the `templates` folder containing manifest templates to  the `manifests` folder and overwrites each placeholder value with what is available in the config file.

11. Transfer badges to auto job account(s). Use `transaction-manifest\manifests\transfer_badges_to_auto_account.rtm` and run that from the develop console.



# TODO

- make function that receives Claim NFT and mints GAT [x]
- Python env to run script locally, docs
- instantiate and fill the rest of config
- generate all manifest (for now leave change metadata out)
- make test cases so far
- manifest for change metadata
- test cases for metadata change
- docs for instantiation and deployment 
include `python3 -m venv .venv` and `source .venv/bin/activate`

