use std::collections::HashMap;

use crate::huffman_binary_tree::Node;

pub fn generate_prefix_table(root:&Box<Node>) -> HashMap<char, String> {
    let mut prefix_table = HashMap::new(); // The prefix table is a HashMap that maps each character to its prefix code
    let mut prefix = String::new();
    generate_prefix_table_recursive(root, &mut prefix, &mut prefix_table);    
    prefix_table
}

fn generate_prefix_table_recursive(node: &Box<Node>, prefix: &mut String, prefix_table: &mut HashMap<char, String>) {

    // If the node is a leaf, insert the character and its prefix code into the prefix table
    if node.character != '\0' {
        prefix_table.insert(node.character, prefix.clone());
    }

    // If the node has a left child, add a '0' to the prefix and call the function recursively
    if let Some(left_child) = &node.left_child {
        prefix.push('0');
        generate_prefix_table_recursive(left_child, prefix, prefix_table);
        prefix.pop();
    }

    // If the node has a right child, add a '1' to the prefix and call the function recursively
    if let Some(right_child) = &node.right_child {
        prefix.push('1');
        generate_prefix_table_recursive(right_child, prefix, prefix_table);
        prefix.pop();
    }
    
}

// Unit tests

#[cfg(test)]
mod tests {
    use crate::huffman_binary_tree::generate_huffman_tree;
    use super::*;

    #[test]
    fn test_generate_prefix_table() {
        let mut freq = HashMap::new();
        freq.insert('c', 32);
        freq.insert('d', 41);
        freq.insert('e', 120);
        freq.insert('k', 7);
        freq.insert('l', 42);
        freq.insert('m', 24);
        freq.insert('u', 37);
        freq.insert('z', 2);
        
        let root = generate_huffman_tree(&freq);
        let prefix_table = generate_prefix_table(&root);

        assert_eq!(prefix_table.get(&'c').unwrap(), "1110");
        assert_eq!(prefix_table.get(&'d').unwrap(), "101");
        assert_eq!(prefix_table.get(&'e').unwrap(), "0");
        assert_eq!(prefix_table.get(&'k').unwrap(), "111101");
        assert_eq!(prefix_table.get(&'l').unwrap(), "110");
        assert_eq!(prefix_table.get(&'m').unwrap(), "11111");
        assert_eq!(prefix_table.get(&'u').unwrap(), "100");
        assert_eq!(prefix_table.get(&'z').unwrap(), "111100");
    }
}