use rust_base58::{ToBase58, FromBase58};
use hex;
use solana_sdk::bs58;

fn main() {
    let address = String::from("9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb");

    let addr_bytes = address.from_base58().unwrap();

    println!("{}",hex::encode(&addr_bytes));
    println!("{:02?}",&addr_bytes);


    let decoded = bs58::decode("9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb").into_vec().unwrap();
    let encoded = bs58::encode(&decoded).into_string();

    assert_eq!(encoded, "9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb");


     let input = "090A0B0C";
     let decoded_hex = hex::decode(input).expect("Decoding failed");

     assert_eq!(decoded_hex,[9, 10, 11, 12])

}

#[test]
fn to_bytes() {
    let x:i32 = 100;
    let x_bytes = x.to_be_bytes();
    let original_x = i32::from_be_bytes(x_bytes);

    assert_eq!(x, original_x);
}



#[test]
fn bytes_vec_to_bs58(){
    let bytes_vec = vec![61,185,226,148,20,20,84,27,65,77,131,77,204,37,205,186,149,240,79,19,150,86,37,11,92,244,199,40,91,227,144,124];
    let encoded_test = bs58::encode(&bytes_vec).into_string();
    println!("{:?}", encoded_test);

}




use borsh::{BorshSerialize, BorshDeserialize};
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct A {
    x: u64,
    y: String,
}

#[test]
fn test_simple_struct() {
    let a = A {
        x: 123,
        y: "hello".to_string(),
    };
    let encoded_a = a.try_to_vec().unwrap();
    let decoded_a = A::try_from_slice(&encoded_a).unwrap();

    println!("{:?}", encoded_a);//[123, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 104, 101, 108, 108, 111]
    println!("{:?}", decoded_a);//A { x: 123, y: "hello" }

    assert_eq!(a, decoded_a);
}
