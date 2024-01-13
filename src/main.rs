mod encode;
use encode::encode;
mod decode;
use decode::decode;

fn main() {
    let x = String::from("happy hip hop");

    // following will give you the compressed bytes
    let encoded_bytes = encode(x).unwrap();

    for bit in encoded_bytes.iter() {
        println!("{:08b}", bit)
    }
    println!();

    let decoded_string = decode(&encoded_bytes).unwrap();
    println!("{}", decoded_string);
}
