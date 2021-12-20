use rust_base58::{ToBase58, FromBase58};
use hex;
use solana_sdk::bs58;

fn main() {
    let address = String::from("9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb");
    let address1 = String::from("25zBh6DbBPQmQUGeLMaEfNAASfvxrm1GvVg3bfTZ6NkG");
    //let x:i32 = 100;
    //let x_bytes = x.to_be_bytes();
    //let original_x = i32::from_be_bytes(x_bytes);

    let hash = address.from_base58().unwrap();
    let hash1 = address1.from_base58().unwrap();


    //println!("{}",hex::encode(&hash));
    println!("{:02x?}", hash);
    println!("{:02x?}", hash1);


    let decoded = bs58::decode("9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb").into_vec().unwrap();
    let encoded = bs58::encode(&decoded).into_string();

    println!("{:02x?}", &decoded);
    assert_eq!(encoded, "9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb");


     let input = "090A0B0C";

     let decoded_hex = hex::decode(input).expect("Decoding failed");

     assert_eq!(decoded_hex,[9, 10, 11, 12])

}
