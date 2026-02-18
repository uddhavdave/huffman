use crate::error::EncodeError;
use bitvec::prelude::*;
use huffman::EncodedData;
use serde::Deserialize;
use serde_cbor::de::Deserializer as CBORDeserializer;
use std::collections::HashMap;

pub fn decode(bytes: &[u8]) -> Result<String, EncodeError> {
    let mut text = String::new();
    let mut cbor_deserializer = CBORDeserializer::from_slice(bytes);
    let encoded_data: EncodedData = Deserialize::deserialize(&mut cbor_deserializer)?;
    let rev_huff_table: HashMap<String, char> =
        HashMap::from_iter(encoded_data.table.map.into_iter().map(|(k, v)| (v, k)));
    let bits: BitVec<u8, Msb0> = BitVec::from_vec(encoded_data.data);

    let mut coding = String::new();
    // Only process the meaningful bits, ignoring any padding in the last byte
    for bit in bits.iter().take(encoded_data.bit_length) {
        if *bit {
            coding += "1"
        } else {
            coding += "0"
        }

        if let Some(value) = rev_huff_table.get(&coding) {
            text.push(*value);
            // clear the coding as the mapped character is appended to the string
            coding.clear();
        }
    }

    if !coding.is_empty() {
        return Err(EncodeError::IndexNotPresent(coding));
    }

    Ok(text)
}
