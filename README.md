# Huffman Encoding Implementation
Huffman Encoding serves as the foundation for various lossless compression algorithms utilized in formats such as WEBP and PNG.
This variable-length encoding method leverages the frequency distribution of letters in data to assign unique codes,
with more common letters receiving shorter encodings, thereby optimizing space utilization.
Modern implementations have further evolved this method by introducing additional patterns to achieve enhanced compression ratios.

## Overview
This repository contains a basic implementation of Huffman Coding compression where
couple of assumptions are made to create a complete proof of concept solution.

You can run this tool via CLI directly. Works similar to `base64` binary in Linux.
Following command is an example of huffman binary used with piped input:
```
echo "TEXT_TO_COMPRESS" | cargo run -- -e | cargo run -- -d
```
Note that `-e`(encrypt) flag will output CBOR data in hex string, and `-d`(decode) flag
expects the input to be a hex string.

## Assumptions
1. The code introduces an End-of-File (EOF) character '■' to signal the decoder to cease reading when this character is encountered.
2. The encoded data is prefixed with the Huffman Table and subsequently serialized in Concise Binary Object Representation (CBOR). 

## TODO

- [ ] Implement multilevel table creation for handling complex data structures.
- [ ] Enhance the algorithm by incorporating pattern recognition and optimizing the encoding process.
- [ ] File Support

Feel free to contribute, provide feedback, or explore ways to further refine and extend this Huffman Encoding implementation.
