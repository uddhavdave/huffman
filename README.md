# Huffman Encoding Implementation

Huffman Encoding serves as the foundation for various lossless compression algorithms utilized in formats such as WEBP and PNG. This variable-length encoding method leverages the frequency distribution of letters in data to assign unique codes, with more common letters receiving shorter encodings, thereby optimizing space utilization. Modern implementations have further evolved this method by introducing additional patterns to achieve enhanced compression ratios.

## Overview

This repository contains a basic implementation of Huffman Encoding compression, providing a proof of concept solution based on a set of assumptions.

## Assumptions

1. The code introduces an End-of-File (EOF) character '■' to signal the decoder to cease reading when this character is encountered.

2. The encoded data is prefixed with the Huffman Table and subsequently serialized in Concise Binary Object Representation (CBOR). The decoder then reverses the Huffman Table and decodes the data. While an alternative approach involves serializing the Huffman tree and traversing it directly, the current implementation follows the convention of using tables for this purpose.

## TODO

- [ ] Implement multilevel table creation for handling complex data structures.
- [ ] Enhance the algorithm by incorporating pattern recognition and optimizing the encoding process.

Feel free to contribute, provide feedback, or explore ways to further refine and extend this Huffman Encoding implementation. Your involvement is valuable in advancing the capabilities of this compression technique.
