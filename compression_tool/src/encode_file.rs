// Function to encode a file

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::collections::HashMap;


fn write_header_to_file(prefix_table: &HashMap<char, String>, output_file_name: &str){
    let mut file = File::create(output_file_name).expect("Error creating file");
    file.write_all(b"PREFIX TABLE:\n").expect("Error writing to file");
    for (key, value) in prefix_table {
        write!(file, "{}:{}\n", key, value).expect("Error writing to file");
    }
    file.write_all(b"PREFIX TABLE END\n").expect("Error writing to file");
}

pub fn encode_file(input_file: &str, output_file: &str, prefix_table: &HashMap<char, String>) {
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
    let mut file = File::create(output_file).expect("Error creating file");

    // Write the encoded text to the output file
    // Open the output file in append mode
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)  // Set append mode
        .create(true)
        .open(output_file)
        .expect("Error opening the file");

    write_header_to_file(prefix_table, output_file);
    file.write_all(&bytes).expect("Error writing to file");
}


// Unit tests

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs;

//     #[test]
//     fn test_encode_file() {
//         let input_file = "input.txt";
//         let output_file = "output.txt";
//         let prefix_table = [('a', "0".to_string()), ('b', "10".to_string()), ('c', "110".to_string()), ('d', "111".to_string())];
//         encode_file(input_file, output_file, &prefix_table);

//         let expected_output = "0101101110";
//         let output = fs::read_to_string(output_file).expect("Error reading the file");
//         assert_eq!(output, expected_output);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_header_to_file() {
        let mut prefix_table = HashMap::new();
        prefix_table.insert('a', "0".to_string());
        prefix_table.insert('b', "10".to_string());
        prefix_table.insert('c', "110".to_string());
        prefix_table.insert('d', "111".to_string());

        write_header_to_file(&prefix_table, "test_prefix_table.txt");
    }
}
