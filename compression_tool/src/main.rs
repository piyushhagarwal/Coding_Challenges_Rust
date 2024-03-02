use compression_tool::{generate_huffman_tree, generate_frequency_table, generate_prefix_table};

fn main() {
    let file_name = "test_file.txt";
    let frequency_table = generate_frequency_table(file_name);
    let root = generate_huffman_tree(&frequency_table);
    let prefix_table = generate_prefix_table(root);

    // Print prefix table
    for (character, prefix) in prefix_table.iter() {
        println!("{}: {}", character, prefix);
    }
}
