// Function to encode a file

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::collections::{HashMap, VecDeque};

use crate::huffman_binary_tree::Node;

fn serialize_huffman_binary_tree(huffman_binary_tree: Option<Box<Node>>) -> String {

    let mut result = String::new();
    // Create queue to store the nodes of the huffman_binary_tree
    let mut queue = VecDeque::new();
    queue.push_back(huffman_binary_tree);
    
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        match node {
            Some(node) => {
                result.push_str(&node.character.to_string());
                result.push_str(" ");
                queue.push_back(node.left_child);
                queue.push_back(node.right_child);
            },
            None => {
                result.push_str("null ");
            }
        }
    }

    // Remove the last space character
    result.pop();
    
    result
}

fn write_header_to_file(huffman_binary_tree: Box<Node>, output_file_name: &str){
    // Serialize the huffman_binary_tree to a string    
    let serialized_huffman_binary_tree = serialize_huffman_binary_tree(Some(huffman_binary_tree));

    // Open the output file in append mode
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)  // Set append mode
        .create(true)
        .open(output_file_name)
        .expect("Error opening the file");

    // Write the serialized huffman_binary_tree to the output file
    file.write_all(serialized_huffman_binary_tree.as_bytes()).expect("Error writing to file");

    // Write a newline character to the output file
    file.write_all(b"\n").expect("Error writing to file");

}

pub fn encode_file(input_file: &str, output_file: &str, huffman_binary_tree: Box<Node>, prefix_table: &HashMap<char, String>) {
    // Read the input file
    let mut file = File::open(input_file).expect("Error opening the file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Error reading the file");


    // Encode the text using the prefix table
    let mut encoded_text = String::new();
    for c in text.chars() {
        encoded_text.push_str(prefix_table.get(&c).unwrap());
    }

    let mut bytes = Vec::new();
    let mut byte = 0u8;
    let mut bit_count = 0;

    // Iterate over each character in the string
    for bit_char in encoded_text.chars() {
        // Shift the byte one bit to the left to make room for the next bit
        byte <<= 1;

        // Set the last bit of the byte if the character is '1'
        if bit_char == '1' {
            byte |= 1;
        }

        bit_count += 1;

        // If we've added 8 bits to the current byte, or we're at the end of the string,
        // add the byte to the vector and start with a new byte
        if bit_count % 8 == 0 || bit_count == encoded_text.len() {
            bytes.push(byte);
            byte = 0; // Reset for the next byte
        }
    }

    // Write the bytes to a text file (with .bin extension)
    File::create(output_file).expect("Error creating file");

    // Write the encoded text to the output file
    // Open the output file in append mode
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)  // Set append mode
        .create(true)
        .open(output_file)
        .expect("Error opening the file");

    write_header_to_file(huffman_binary_tree, output_file);

    file.write_all(&bytes).expect("Error writing to file");
}


