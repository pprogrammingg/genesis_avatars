use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct GatData {
    ipfs_id: String,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ClaimData {
    claimable_after_instant: Instant,
}

#[blueprint]
mod genesis_avatar {
    struct GenesisAvatar {
        // Vault to store and freeze GENAVS deposits from users
        genav_frozen_vault: Vault,
        gat_resource_manager: ResourceManager,
        claim_badge_resource_manager: ResourceManager,
        claimable_after_days: u8,
        sequence: u16,
    }

    impl GenesisAvatar {
        /**
         * instantiate()
         * 1. Create resource manager for Genesis Avatar Token (GAT NFT)
         *  a. base_url of the NFT collection will be set in Metadata
         *  b. metadata can be updated and eventually be locked via a certain badge
         * 2. Create resource manager for Claim NFTS (NFTs issued in exchange of GENAV which in turn will be used later by user to claim GAT)
         * 3. Create resource manager for Metada Updater Badge. Mint 1 and depost to instantiaor account.
         * 4. Create a vault to freeze deposits of GENAV (legacy tokens that need to be swapped with GAT)
         */
        pub fn instantiate() -> FungibleBucket {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(GenesisAvatar::blueprint_id());

            // 1.
            let gat_resource_manager = ResourceBuilder::new_ruid_non_fungible::<GatData>(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Genesis Avatar Token".to_owned(), locked;
                        "symbol" => "GAT".to_owned(), locked;
                        "description" => "Genesis Avatar NFT which are playable NFTs in the game".to_owned(), locked;
                        "base_url" => "place_holder_ipfs_url".to_owned(), locked;
                    }
                })
                .mint_roles(mint_roles! {
                    minter => rule!(require(global_caller(component_address))); 
                    minter_updater => rule!(deny_all);
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(global_caller(component_address))); 
                    burner_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            // Definition of Claim Badge NFT
            let claim_badge_resource_manager = ResourceBuilder::new_ruid_non_fungible::<ClaimData>(OwnerRole::None)
            .metadata(metadata! {
                init {
                    "name" => "Claim Badge".to_owned(), locked;
                    "symbol" => "CAIM_BADGE".to_owned(), locked;
                    "description" => "Claim badge NFT are a placeholder for GAT NFTs.".to_owned(), locked;
                }
            })
            .mint_roles(mint_roles! {
                minter => rule!(require(global_caller(component_address))); 
                minter_updater => rule!(deny_all);
            })
            .burn_roles(burn_roles! {
                burner => rule!(require(global_caller(component_address))); 
                burner_updater => rule!(deny_all);
            })
            .create_with_no_initial_supply();

            let gat_metadata_updater_badge = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_NONE)
                .metadata(metadata!(
                    init {
                        "name" => "GAT Metadata Updater Badge".to_owned(), locked;
                        "symbol" => "GMUB".to_owned(), locked;
                        "description" => "Used to update GAT metadata".to_owned(), locked;
                    }
                ))
                .mint_roles(mint_roles! {
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                })
                .mint_initial_supply(Decimal::one());
            
            let genav_resource_address = ResourceAddress::try_from_bech32(&AddressBech32Decoder::new(&NetworkDefinition::stokenet()),"resource_rdx1t5rfs8q0jsl3jn3vxn8h9r2a4h9kvyrcl5a0ee9g2gq9dmcwt826dl").unwrap().to_vec();

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            Self {
                genav_frozen_vault: Vault::new(ResourceAddress.from_str("resource_rdx1t5rfs8q0jsl3jn3vxn8h9r2a4h9kvyrcl5a0ee9g2gq9dmcwt826dl")),
                gat_resource_manager,
                claim_badge_resource_manager,
                claimable_after_days: 3u8,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .with_address(address_reservation)
            .globalize();

            gat_metadata_updater_badge
        }

        /*
         * mint_claim_nft_given_genav
         *
         * Given GENAV tokens,for each token
         * 1. Mint a claim NFT
         * 2. Set claim NFT data to have:
         *  a. claimable_after_date : equal to current instant
         *  b. sequence: an incremental sequence number tracking the order of mint.
         * Returns claim NFT to the user
         */
        pub fn mint_claim_nft_given_genav(&mut self, genav_bucket: Bucket) -> Bucket {
            let genav_amount = genav_bucket.amount();

            self.genav_frozen_vault.put(genav_bucket);

            for i in 0..=genav_amount {
                self.sequence += 1;
            
                let new_claim_data = ClaimNFT {
                    sequence: self.sequence,
                    issued_on_instant: Clock::current_time_rounded_to_minutes(),
                };


            }
                              
        }
    }
}
