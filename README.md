# Huffman Coding
Huffman Coding is the base of all lossless compression algorithms used in formats
like WEBP and PNG. This variable length encoding method uses the disparity of frequencies
of letters in data to assign unique codes to each letter where the most common letter
is assigned a shorter encoding which saves space and vice versa. Modern implementations
have added on top of this method and added patterns to the encoding to achieve even
greater compression.

This repository contains a basic implementation of Huffman Coding compression where
couple of assumptions are made to create a complete proof of concept solution.

## Assumptions
- This code adds an EOF character '■' to indicate the decoder to stop reading once
this character is encountered.
- The encoded data is prepended with the Huffman Table and then serialized in CBOR.
Decoder then reverses the Huffman Table and decodes the data. Arguably, we can also
serialize the Huffman tree and traverse it directly for a cleaner approach but it
seems to be a norm to use tables for this.

## TODO
- [] Add multilevel table creation for complex data
- [] Improve the algorithm by recognizing patterns and encoding them
