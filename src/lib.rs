use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct GatData {
    ipfs_id: String,
}

#[derive(ScryptoSbor, NonFungibleData)]
pub struct ClaimData {
    sequence: u16,
    issued_on_instant: Instant,
}

#[blueprint]
mod genesis_avatar {
    struct GenesisAvatar {
        // Vault to store GENAVS deposits from users
        genav_vault: Vault,
        gat_resource_manager: ResourceManager,
        claim_badge_resource_manager: ResourceManager,
        claimable_after_days: u8,
        sequence: u16,
        ipfs_id_assignments: [u16; 10001],
        test_genav_resource_manager: ResourceManager,
    }

    impl GenesisAvatar {
        ///
        // instantiate
        // 1. Create resource manager for Metada Updater Badge. Mint 1 and depost to instantiaor account.
        //      a. Define an access rule based on the metadata updater badge
        //      b. Use this rule for metadata setting of GAT NFT resourcemanager
        // 2. Create resource manager for Genesis Avatar Token (GAT NFT)
        // 3. Create resource manager for Claim NFTS (NFTs issued in exchange of GENAV which in turn will be used later by user to claim GAT)
        // 4. Create a vault to keep deposits of GENAV (legacy tokens that need to be swapped with GAT)
        //
        pub fn instantiate() -> FungibleBucket {
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(GenesisAvatar::blueprint_id());

            // 1.
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
            
            // 1a.
            let metadata_setter_access_rule: AccessRule =
                rule!(require(gat_metadata_updater_badge.resource_address()));

            // 2.
            let gat_resource_manager = ResourceBuilder::new_ruid_non_fungible::<GatData>(OwnerRole::None)
                .metadata(metadata! {
                    roles { // 1b.
                        metadata_locker => metadata_setter_access_rule.clone();
                        metadata_locker_updater => metadata_setter_access_rule.clone();
                        metadata_setter => metadata_setter_access_rule.clone();
                        metadata_setter_updater => metadata_setter_access_rule;
                    },
                    init {
                        "name" => "Genesis Avatar Token".to_owned(), locked;
                        "symbol" => "GAT".to_owned(), locked;
                        "description" => "Genesis Avatar NFT which are playable NFTs in the game".to_owned(), locked;
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

            // 3.
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

            // TODO temporary for testing only, rm after
            let test_genav_resource_manager = ResourceBuilder::new_fungible(OwnerRole::None)
                .divisibility(DIVISIBILITY_MAXIMUM)
                .metadata(metadata!(
                    init {
                        "name" => "TGENAV".to_owned(), locked;
                        "symbol" => "TGENAV".to_owned(), locked;
                        "description" => "Used to test GENAV".to_owned(), locked;
                    }
                ))
                .mint_roles(mint_roles! {
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            // TODO: uncomment for prod
            // let genave_resource_address = ResourceAddress::try_from_bech32(
            //     &AddressBech32Decoder::new(&NetworkDefinition::mainnet()),
            //     "resource_rdx1t5rfs8q0jsl3jn3vxn8h9r2a4h9kvyrcl5a0ee9g2gq9dmcwt826dl",
            // )
            // .unwrap();
       
            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            Self {
                genav_vault: Vault::new(test_genav_resource_manager.address()), // 4.
                gat_resource_manager,
                claim_badge_resource_manager,
                claimable_after_days: 3u8,
                sequence: 0u16,
                // ipfs_id_assignments : initialize with 0. Value of 0 means ipfs_id not assigned, 1 means assigned. 
                // Using 10_001 as array length helps, since we can set 1 or 0 for indecies 0 through 10000. This way,
                // can check ipfs_id_assignments[123] or ipfs_id_assignments[10000] without any other index manipulation. 
                // Note: ipfs_id_assignments[0] is not important.
                ipfs_id_assignments: [0; 10001], 
                test_genav_resource_manager,
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
         * 0. Take the integer part of genav_bucket 
         *  0a. If integer amount is less than 1 throw error
         * 1. Store input genav tokens in to genav_vault
         * 2. Set claim NFT data to have:
         *  2a. issued_on_instant : equal to current instant
         *  2b. sequence: an incremental sequence number tracking the order of mint.
         * 3. Mint a claim NFT
         * Returns claim NFT to the user
         */
        pub fn mint_claim_nft_given_genav(&mut self, genav_bucket: Bucket) -> Bucket {
            let mut claim_nfts_bucket = Bucket::new(self.claim_badge_resource_manager.address());
            
            // 0.
            let genav_amount = genav_bucket.amount();
            
            // 0a.
            assert!(genav_amount >= dec!(1.0), "[ERR_INVALID_GENAV_AMOUNT: invalid amount of GENAV tokens is provided. Make sure amount is greater than 1!");

            // 0b.
            // Assumption is that whole GENAV tokens will be provided, 
            // but in case an amount with decimals is provided we take a floor
            let genav_floor_amount = match genav_amount.checked_floor() {
                Some(amount) => i32::try_from(amount).unwrap(),
                None => {
                    panic!("[ERR_FAILED_GET_INTEGER_GENAV_AMOUNT: could not extract integer amount from decimal genav amount provided!");
                }
            };

            // 1.
            self.genav_vault.put(genav_bucket);

            for _i in 0..genav_floor_amount {
                self.sequence += 1;
                
                // 1.
                let new_claim_data = ClaimData {
                    sequence: self.sequence, // 1a.
                    issued_on_instant: Clock::current_time_rounded_to_minutes(), // 1b.
                };

                // 2.
                claim_nfts_bucket.put(
                    self.claim_badge_resource_manager
                        .mint_ruid_non_fungible(new_claim_data),
                );
            }

            claim_nfts_bucket
        }

        /** 
         * Given a claim NFT and ipfs_id
         *  1. Validations
         *  1a. Assert ipfs_id is between 1 and 10000 inclusive
         *  1b. Assert ipfs_id is not already assigned to another NFT.
         *  1c. Assert claim NFT has a valid resource address
         *  2. Assert claim NFT is redeemable based on adding a pre-determined number of days (e.g. 10) to claim_issued_on_instant field
         *  3. burn the input claim NFT
         *  4. mint new GAT with ipfs_id as its data
         *  5. Set ipfs_id_assignments to value one for index being integer value of ipfs_id
         * Returns a GAT NFT
         */
        pub fn mint_gat_given_claim_nft(&mut self, claim_nft_bucket: NonFungibleBucket, ipfs_id: String) -> Bucket {

            // 1.

            let ipfs_id_u16 = ipfs_id.parse::<u16>().unwrap();

            // 1a.
            assert!(ipfs_id_u16 >= 1 && ipfs_id_u16 <= 10000,
                "[ERR_INVALID_IPFS_ID_RANGE] Invalid range for ipfs_id provided, must be between 1 and 10000!");

            // 1b.
            assert!(self.ipfs_id_assignments[ipfs_id_u16 as usize] == 0, 
                "[ERR_IPFS_ID_ALREADY_ASSIGNED] IPFS_ID provided is already assigned to another NFT!"); 

            // 1c.
            assert!(claim_nft_bucket.resource_address() == self.claim_badge_resource_manager.address(), 
                "[ERR_INVALID_CLAIM_NFT] Invalid claim NFT provided!");
            
            let claim_nft: NonFungible<ClaimData> = claim_nft_bucket.as_non_fungible().non_fungible();

            let data: ClaimData = claim_nft.data();
            let claim_issued_on_instant: Instant = data.issued_on_instant;
            let claimable_after_instant = claim_issued_on_instant.add_days(3).unwrap();
            let claim_redeemable_on_instant: Instant = claimable_after_instant; 
            // 2.
            assert!(Clock::current_time_is_at_or_after(claim_redeemable_on_instant, TimePrecision::Minute), 
                "[ERR_CLAIM_NFT_NOT_REDEEMABLE] Too soon to process claim NFT. It is claimable after {:?}.", claimable_after_instant);
    
            // 3.
            claim_nft_bucket.burn();

            // 4.
            let new_gat_data = GatData {
                ipfs_id: ipfs_id
            };

            // 5.
            self.ipfs_id_assignments[ipfs_id_u16 as usize] = 1;

            self.gat_resource_manager.mint_ruid_non_fungible(new_gat_data)
        }

        pub fn mint_test_genav(&mut self) -> Bucket {
            self.test_genav_resource_manager.mint(50)
        }
    }
}
