use std::io::{self, Read, Write, BufRead, BufReader};
use std::collections::HashMap;
use std::fs::File;

// Function to read the header from the file and construct the prefix table
fn read_header_from_file(input_file: &str) -> io::Result<HashMap<String, char>> {
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    // Parse the header to construct the prefix table
    let mut prefix_table = HashMap::new();
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        if line == "PREFIX TABLE END" {
            break;
        }

        if line == "PREFIX TABLE:" {
            continue;
        }

        let parts: Vec<&str> = line.split('$').collect();
        if parts.len() == 2 {
            let key = parts[0].chars().next().unwrap_or_else(|| {
                eprintln!("Error parsing key in line: {}", line);
                panic!("Unexpected header key format");
            });
            let value = parts[1].to_string();
            prefix_table.insert(value, key);
        } else {
            eprintln!("Error parsing header line: {}", line);
            panic!("Unexpected header format");
        }
    }

    Ok(prefix_table)
}

// Function to decode the file using the provided prefix table
pub fn decode_file(input_file: &str, output_file: &str) -> io::Result<()> {
    // Read the header to reconstruct the prefix table
    let prefix_table = read_header_from_file(input_file)?;

    // Print the reconstructed prefix table
    println!("Reconstructed Prefix Table:");
    for (key, value) in &prefix_table {
        println!("{}: {}", key, value);
    }

    // Open the input file
    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);

    // Skip the header
    let mut line = String::new();
    loop {
        reader.read_line(&mut line)?;
        if line == "PREFIX TABLE END\n" {
            break;
        }
        line.clear();
    }

    // Read the encoded binary data
    let mut encoded_data = Vec::new();
    reader.read_to_end(&mut encoded_data)?;

    // print the encoded data
    println!("Encoded Data: {:?}", encoded_data);

    // Decode the binary data using the prefix table
    let mut decoded_text = String::new();
    let mut current_code = String::new();

    for byte in encoded_data.iter() {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;
            current_code.push_str(&bit.to_string());

            if let Some(&character) = prefix_table.get(&current_code) {
                decoded_text.push(character);
                current_code.clear();
            }
        }
    }

    // Write the decoded text to the output file
    let mut output_file = File::create(output_file)?;
    output_file.write_all(decoded_text.as_bytes())?;

    Ok(())
}