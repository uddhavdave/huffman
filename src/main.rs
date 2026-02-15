mod encode;
use encode::encode;
mod decode;
mod error;
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
                hex::decode(String::from_utf8(buffer).expect("Invalid UTF-8 input").trim())
                    .expect("Invalid hex-encoded input");
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

fn read_input(buffer: &mut Vec<u8>) {
    if let Err(err) = std::io::stdin().read_to_end(buffer) {
        eprintln!("Error reading from stdin: {}", err);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn roundtrip(input: &str) {
        let encoded_bytes = encode(input).unwrap();
        let decoded_string = decode(&encoded_bytes).unwrap();
        assert_eq!(input, decoded_string);
    }

    #[test]
    fn test_encode_decode() {
        roundtrip("happy hip hop");
    }

    #[test]
    fn test_single_character() {
        roundtrip("a");
    }

    #[test]
    fn test_repeated_character() {
        roundtrip("aaaaaaa");
    }

    #[test]
    fn test_two_characters() {
        roundtrip("ab");
    }

    #[test]
    fn test_long_string() {
        roundtrip("the quick brown fox jumps over the lazy dog");
    }

    #[test]
    fn test_unicode() {
        roundtrip("hello world 你好世界");
    }

    #[test]
    fn test_whitespace_and_newlines() {
        roundtrip("line one\nline two\ttab");
    }
}
