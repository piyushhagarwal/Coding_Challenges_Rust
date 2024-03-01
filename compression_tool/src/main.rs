use compression_tool::{generate_huffman_tree, generate_frequency_table};
use std::collections::HashMap;



fn main() {
    let file_name = "test_file.txt";
    let frequency_table = generate_frequency_table(file_name);
    let root = generate_huffman_tree(&frequency_table);

}
