use compression_tool::{generate_huffman_tree, generate_frequency_table, generate_prefix_table, encode_file, decode_file};

fn main() {
    let file_name = "input.txt";
    let frequency_table = generate_frequency_table(file_name);
    let root = generate_huffman_tree(&frequency_table);
    let prefix_table = generate_prefix_table(&root);

    encode_file(file_name, "encoded_file.bin", &prefix_table);
    decode_file("encoded_file.bin", "decoded_file.txt").unwrap();
    
}