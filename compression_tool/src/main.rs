use compression_tool::{generate_huffman_tree, Node, inorder};
use std::collections::HashMap;



fn main() {
    let mut freq = HashMap::new();
    freq.insert('c', 32);
    freq.insert('d', 42);
    freq.insert('e', 120);
    freq.insert('k', 7);
    freq.insert('l', 42);
    freq.insert('m', 24);
    freq.insert('u', 37);
    freq.insert('z', 2);

    let root = generate_huffman_tree(&freq);
    inorder(&Some(Box::new(root)));
}
