
use genesis_avatar::test_bindings::*;
use scrypto::*;
use scrypto_test::prelude::*;

#[test]
fn get_bech_32_address() -> Result<(), RuntimeError> {

    // let my_resource_address = ResourceAddress::try_from_hex("5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6").unwrap();

    // let encoder = AddressBech32Encoder::new(&NetworkDefinition::mainnet());
    // let my_bech32_address = encoder.encode(my_resource_address.as_ref()).unwrap();


    // assert!(2==3, "{my_bech32_address}");
    // let ipfs_id = "123".to_owned();
    // let ipfs_id_u16 = ipfs_id.parse::<u16>().unwrap();
    // let bool_res = ipfs_id_u16 >= 1 || ipfs_id_u16 <= 10000;
    //assert!(2==3, "answer is {bool_res}");

    // let mut ipfs_array: [u16; 3] = [0;3];

    // ipfs_array[1u16 as usize] = 1;

    // println!("ipfs_array is {:?}", ipfs_array);

    let genav_amount = dec!(2.3);

    let genav_floor_amount = match genav_amount.checked_floor() {
        Some(amount) => i32::try_from(amount).unwrap(),
        None => {
            panic!("[ERR_INVALID_GENAV_AMOUNT: invalid amount of GENAV tokens is provided. Make sure amount is greater than 1!");
        }
    };

    println!("genav_floor_amount is {:?}", genav_floor_amount);

    Ok(())
}