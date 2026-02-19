use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct EncodedData {
    pub table: HuffTable,
    pub data: Vec<u8>,
    /// Number of meaningful bits in `data`. The last byte may contain padding bits.
    pub bit_length: usize,
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
                // Leaf node, hence save the bit encoding in the table.
                // If coding is empty, this is the only character (root is a leaf),
                // so assign it a single-bit code.
                let code = if coding.is_empty() {
                    "0".to_string()
                } else {
                    coding
                };
                table.map.insert(self.character.unwrap(), code);
            }
            [Some(left), Some(right)] => {
                // Traverse inorder
                left.create_table_from_huff_tree(table, coding.clone() + "0");
                right.create_table_from_huff_tree(table, coding + "1");
            }
            _ => {}
        }
    }

    pub fn print_tree(&self, prefix: &str, is_left: bool, is_root: bool) {
        let connector = if is_root {
            ""
        } else if is_left {
            "├── "
        } else {
            "└── "
        };

        let label = if let Some(ch) = self.character {
            let display = match ch {
                ' ' => "SP".to_string(),
                '\n' => "LF".to_string(),
                '\t' => "TAB".to_string(),
                c => format!("'{}'", c),
            };
            format!("{} (freq: {})", display, self.freq)
        } else {
            format!("[{}]", self.freq)
        };

        eprintln!("{}{}{}", prefix, connector, label);

        let child_prefix = if is_root {
            String::new()
        } else {
            format!("{}{}", prefix, if is_left { "│   " } else { "    " })
        };

        if let [Some(left), Some(right)] = &self.child {
            left.print_tree(&child_prefix, true, false);
            right.print_tree(&child_prefix, false, false);
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
