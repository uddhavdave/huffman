use crate::encode::{build_freq_map, build_tree, encode_with_table};
use huffman::PSEUDO_EOF_CHAR;

pub fn visualize(input: &str) {
    let mut text = input.to_string();
    text.push(PSEUDO_EOF_CHAR);

    let freq_map = build_freq_map(&text);
    let huff_tree = build_tree(&freq_map);

    // Print frequency table sorted by frequency (descending)
    eprintln!("--- Frequency Table ---");
    let mut freq_list: Vec<_> = freq_map.iter().collect();
    freq_list.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (ch, count) in &freq_list {
        let display = format_char(**ch);
        eprintln!("  {:>5}  {:>3}", display, count);
    }
    eprintln!();

    // Print huffman tree
    eprintln!("--- Huffman Tree ---");
    huff_tree.print_tree("", true, true);
    eprintln!();

    // Build table from tree (this consumes the tree)
    let huff_table = huff_tree.get_huff_table();

    // Print code table sorted by code length, then char
    eprintln!("--- Code Table ---");
    let mut codes: Vec<_> = huff_table.map.iter().collect();
    codes.sort_by(|a, b| a.1.len().cmp(&b.1.len()).then(a.0.cmp(b.0)));
    for (ch, code) in &codes {
        let display = format_char(**ch);
        eprintln!("  {:>5}  {:>width$}  ({} bits)", display, code, code.len(), width = codes.last().map_or(1, |c| c.1.len()));
    }
    eprintln!();

    // Compression stats
    let original_bits = input.len() * 8;
    let bv = encode_with_table(&text, &huff_table).unwrap();
    let compressed_bits = bv.len();
    let ratio = if original_bits > 0 {
        (compressed_bits as f64 / original_bits as f64) * 100.0
    } else {
        0.0
    };
    eprintln!("--- Compression Stats ---");
    eprintln!("  Original:   {} bytes ({} bits)", input.len(), original_bits);
    eprintln!("  Compressed: {} bits", compressed_bits);
    eprintln!("  Ratio:      {:.1}%", ratio);
    eprintln!();
}

fn format_char(ch: char) -> String {
    match ch {
        ' ' => "SP".to_string(),
        '\n' => "LF".to_string(),
        '\t' => "TAB".to_string(),
        c if c == PSEUDO_EOF_CHAR => "EOF".to_string(),
        c => format!("'{}'", c),
    }
}
