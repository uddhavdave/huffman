use anyhow::Result;
use bitvec::prelude::*;
use huffman::{EncodedData, HuffTree, PSEUDO_EOF_CHAR};
use serde::ser::Serialize as SerializeTrait;
use serde_cbor::ser::Serializer as CBORSerializer;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[cfg(feature = "alloc")]
pub fn encode(mut text: String) -> Result<Vec<u8>> {
    // We add a Psuedo EOF to the string
    // This will indicate end of stream while decompression
    text.push(PSEUDO_EOF_CHAR);

    let mut bv = bitvec![u8, Msb0;];
    let mut freq_map: HashMap<char, u32> = HashMap::new();
    for i in text.chars() {
        freq_map.entry(i).and_modify(|val| *val += 1).or_insert(1);
    }

    // Minimun Frequency Queue ensures that the first two elements in the queue
    // are the least occuring characters.
    let mut min_freq_pqueue: BinaryHeap<Reverse<HuffTree>> = BinaryHeap::new();
    for (letter, count) in freq_map.iter() {
        let new_node = HuffTree {
            freq: *count,
            character: Some(*letter),
            child: [None, None],
        };
        min_freq_pqueue.push(Reverse(new_node))
    }

    // Keep merging till the queue has only one element which will be the huffman
    // tree. First two elements are merged in each iteration to create a subtree
    // which will then, according to its tree frequency will be merged further.
    // This will generate the most optimal tree for the input.
    while min_freq_pqueue.len() >= 2 {
        let new_node = min_freq_pqueue
            .pop()
            .unwrap()
            .0
            .merge(min_freq_pqueue.pop().unwrap().0);
        min_freq_pqueue.push(Reverse(new_node))
    }

    let huff_tree: HuffTree = min_freq_pqueue.pop().unwrap().0;
    let huff_table = huff_tree.get_huff_table();

    for letter in text.chars() {
        if let Some(coding) = huff_table.map.get(&letter) {
            println!("Letter : {} , Coding : {}", letter, coding);
            for bit in coding.chars() {
                match bit {
                    '0' => bv.push(false),
                    '1' => bv.push(true),
                    _ => panic!(),
                }
            }
        }
    }

    // Structure for encoding the data along with the Huffman table.
    let data = EncodedData {
        table: huff_table,
        data: bv.into_vec(),
    };

    // Serialize in CBOR Format
    let mut cbor_serializer = CBORSerializer::new(Vec::new());
    data.serialize(&mut cbor_serializer)?;

    Ok(cbor_serializer.into_inner())
}
