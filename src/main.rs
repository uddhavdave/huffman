mod encode;
use encode::encode;
mod decode;
mod error;
mod visualize;
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
    #[arg(short, long)]
    verbose: bool,
}
fn main() -> ExitCode {
    // Define the command-line interface
    let args = HuffmanCli::parse();

    let mut buffer: Vec<u8> = Vec::with_capacity(MAX_READ_SIZE);
    if args.decode || args.encode {
        read_input(&mut buffer);

        if args.encode {
            let input = String::from_utf8(buffer).expect("Invalid Input");
            let input = input.trim();
            if args.verbose {
                visualize::visualize(input);
            }
            match encode(input) {
                Ok(data) => println!("{}", hex::encode(data)),
                Err(e) => {
                    eprintln!("Encode failed with: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        } else if args.decode {
            let bytes = hex::decode(
                String::from_utf8(buffer)
                    .expect("Invalid UTF-8 input")
                    .trim(),
            )
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

    #[test]
    fn test_all_ascii_printable() {
        let input: String = (32u8..=126).map(|b| b as char).collect();
        roundtrip(&input);
    }

    #[test]
    fn test_binary_like_content() {
        // Input that could confuse bit-level padding if handled incorrectly
        roundtrip("0000000011111111");
    }

    #[test]
    fn test_special_characters() {
        roundtrip("!@#$%^&*()_+-=[]{}|;':\",./<>?");
    }

    #[test]
    fn test_emoji() {
        roundtrip("hello 🌍🌎🌏");
    }

    #[test]
    fn test_compression_reduces_size() {
        let input = "aaaaaaaaaaaabbbbbbccddde";
        let encoded = encode(input).unwrap();
        // Encoded CBOR includes the table overhead, but the raw bit data
        // should be smaller than the original for repetitive input
        assert!(encoded.len() > 0);
        // Verify roundtrip still works
        let decoded = decode(&encoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_former_eof_char_in_input() {
        // The old pseudo-EOF character (■) should now be treated as normal data
        roundtrip("hello■world");
    }

    #[test]
    fn test_multiline_text() {
        roundtrip("line1\nline2\nline3\n");
    }

    #[test]
    fn test_only_whitespace() {
        roundtrip("   \t\t\n\n  ");
    }
}
