use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This constnant is used as an end of file character which will be useful in decoding.
pub const PSEUDO_EOF_CHAR: char = '■';

#[derive(Serialize, Deserialize, Default)]
pub struct EncodedData {
    pub table: HuffTable,
    pub data: Vec<u8>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HuffTable {
    pub map: HashMap<char, String>,
}

impl HuffTable {
    pub fn new() -> Self {
        HuffTable {
            map: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct HuffTree {
    pub freq: u32,
    pub character: Option<char>,
    pub child: [Option<Box<HuffTree>>; 2],
}

impl HuffTree {
    pub fn merge(self, other: Self) -> Self {
        let new_freq = self.freq + other.freq;
        let character = None;
        let child = [Some(Box::new(self)), Some(Box::new(other))];

        HuffTree {
            freq: new_freq,
            character,
            child,
        }
    }

    pub fn get_huff_table(self) -> HuffTable {
        let mut table = HuffTable::new();

        self.create_table_from_huff_tree(&mut table, String::new());

        table
    }

    fn create_table_from_huff_tree(self, table: &mut HuffTable, coding: String) {
        match self.child {
            [None, None] => {
                // Leaf node, hence save the bit encoding in the table
                table.map.insert(self.character.unwrap(), coding);
            }
            [Some(left), Some(right)] => {
                // Traverse inorder
                left.create_table_from_huff_tree(table, coding.clone() + "0");
                right.create_table_from_huff_tree(table, coding + "1");
            }
            _ => {}
        }
    }
}

impl Ord for HuffTree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.cmp(&other.freq)
    }
}

impl PartialOrd for HuffTree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffTree {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Eq for HuffTree {}
