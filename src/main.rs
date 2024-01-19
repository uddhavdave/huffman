mod encode;
use encode::encode;
mod decode;
mod error;
use clap::command;
use clap::Parser;
use decode::decode;
use std::{io::Read, process::ExitCode};

const MAX_READ_SIZE: usize = 4096;

#[derive(Parser, Debug)]
#[command(name = "Huffman CLI", about = "Data Encoder using Huffman Coding")]
struct HuffmanCli {
    #[arg(short, long)]
    encode: bool,
    #[arg(short, long)]
    decode: bool,
}
fn main() -> ExitCode {
    // Define the command-line interface
    let args = HuffmanCli::parse();

    let mut buffer: Vec<u8> = Vec::with_capacity(MAX_READ_SIZE);
    if args.decode || args.encode {
        read_input(&mut buffer);

        if args.encode {
            match encode(String::from_utf8(buffer).expect("Invalid Input").trim()) {
                Ok(data) => println!("{}", hex::encode(data)),
                Err(e) => {
                    eprintln!("Encode failed with: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        } else if args.decode {
            let bytes =
                hex::decode(String::from_utf8(buffer).expect("asdf").trim()).expect("Hex format");
            match decode(&bytes) {
                Ok(data) => println!("{}", data),
                Err(e) => {
                    eprintln!("Decode failed with: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        }
    } else {
        eprintln!("Either 'encode' or 'decode' must be specified.");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

// Helper function to read input from stdin
fn read_input(buffer: &mut Vec<u8>) -> Vec<u8> {
    if let Err(err) = std::io::stdin().read_to_end(buffer) {
        eprintln!("Error reading from stdin: {}", err);
        std::process::exit(1);
    }
    buffer.clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let x = String::from("happy hip hop");

        // following will give you the compressed bytes
        let encoded_bytes = encode(&x).unwrap();

        for bit in encoded_bytes.iter() {
            println!("{:08b}", bit)
        }
        println!();

        let decoded_string = decode(&encoded_bytes).unwrap();
        println!("{}", decoded_string);

        assert_eq!(x, decoded_string);
    }
}
