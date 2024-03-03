use std::{char, collections::VecDeque, fs::File, io::{BufRead, BufReader, Read, Write}};

use crate::huffman_binary_tree;

struct Node {
    character: char,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>
}

impl Node {
    fn new(character: char) -> Node {
        Node {
            character,
            left_child: None,
            right_child: None
        }
    }
}

fn deserialize_huffman_binary_tree_header(serialized_huffman_binary_tree: &str) -> Option<Box<Node>> {
    let nodes: Vec<&str> = serialized_huffman_binary_tree.split(" ").collect();
    
    if nodes.is_empty() {
        return None;
    }

    let mut queue = VecDeque::new();

    let mut root = Box::new(Node::new('\0'));
    queue.push_back(&mut root);
    let mut i = 0;
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        if i + 1 < nodes.len() && nodes[i + 1] != "null" {
            let char = if nodes[i + 1].is_empty() {
                '\0'
            } else {
                nodes[i + 1].chars().next().unwrap()
            };
            let left_child = Box::new(Node::new(char));
            node.left_child = Some(left_child);
            queue.push_back(node.left_child.as_mut().unwrap());
        }
        i += 1;
        if i + 1 < nodes.len() && nodes[i + 1] != "null" {
            let char = if nodes[i + 1].is_empty() {
                '\0'
            } else {
                nodes[i + 1].chars().next().unwrap()
            };
            let right_child = Box::new(Node::new(char));
            node.right_child = Some(right_child);
            queue.push_back(node.right_child.as_mut().unwrap());
        }
        i += 1;
    }

    Some(root)
}

pub fn decode_file(header_file: &str, encorded_file: &str, decorded_file: &str) {
    // Open the input file
    let file = File::open(header_file).expect("Error opening the file");
    let mut reader = BufReader::new(file);

    // Read the header
    let mut lines = reader.lines();
    let mut header = String::new();
    while let Some(Ok(line)) = lines.next() {
        header.push_str(&line);
    }
    // Print the header
    println!("{}", header);

    let mut huffman_binary_tree = deserialize_huffman_binary_tree_header(&header);

    // Open the input file
    let file = File::open(encorded_file).expect("Error opening the file");
    let mut reader = BufReader::new(file);

    // Read the encoded data
    let mut encoded_data = Vec::new();
    reader.read_to_end(&mut encoded_data).expect("Error reading the file");

    println!("{:?}", encoded_data);

    // Decode the binary data using the prefix table
    let mut decoded_text = String::new();
    let mut current_node = huffman_binary_tree.as_mut().unwrap();

    for byte in encoded_data.iter(){
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;
            
            if current_node.character != '\0' {
                decoded_text.push(current_node.character);
                current_node = huffman_binary_tree.as_mut().unwrap();
            }
            if bit == 0 {
                current_node = current_node.left_child.as_mut().unwrap();
            } else {
                current_node = current_node.right_child.as_mut().unwrap();
            }
        }
    }

    // Write the decoded text to the output file
    let mut file = File::create(decorded_file).expect("Error creating the file");
    file.write_all(decoded_text.as_bytes()).expect("Error writing to file");
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_huffman_binary_tree() {
        let serialized_huffman_binary_tree = " e  null null   u l d  null null null null null null c  null null  m z k null null null null null null";
        let root = deserialize_huffman_binary_tree_header(serialized_huffman_binary_tree);
    }
}