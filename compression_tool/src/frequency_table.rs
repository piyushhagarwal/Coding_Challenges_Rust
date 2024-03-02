use std::fs::File; // Module to work with files
use std::io::Read; // Module to read from files
use std::collections::HashMap; // Module to work with hashmaps

pub fn generate_frequency_table(file_name : &str) -> HashMap<char, i32>{
    // Open the file
    let mut file = File::open(file_name).expect("Problem opening the file");

    // Read from the file
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Create a hashmap to store the frequency of each character
    let mut frequency_map = HashMap::new();

    // Iterate over the characters of the file
    for c in contents.chars() {
        // If the character is not in the hashmap, add it with a frequency of 1
        if !frequency_map.contains_key(&c) {
            frequency_map.insert(c, 1);
        } else {
            // If the character is already in the hashmap, increment its frequency
            let count = frequency_map.get_mut(&c).unwrap();
            *count += 1;
        }
    }

    // Return the frequency_map
    frequency_map
}

// Test cases

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_frequency_table() {
        let file_name = "test_file.txt";
        let frequency_map = generate_frequency_table(file_name);
        assert_eq!(frequency_map.get(&'X'), Some(&333));
        assert_eq!(frequency_map.get(&'t'), Some(&223000));
        assert_eq!(frequency_map.get(&'\0'), None);
    }

    #[test]
    fn test_generate_frequency_table2() {
        let file_name = "input.txt";
        let frequency_map = generate_frequency_table(file_name);
        assert_eq!(frequency_map.get(&'a'), Some(&4));
        assert_eq!(frequency_map.get(&'b'), Some(&5));
        assert_eq!(frequency_map.get(&'c'), Some(&6));
        assert_eq!(frequency_map.get(&'d'), Some(&7));
        assert_eq!(frequency_map.get(&' '), Some(&2));
        assert_eq!(frequency_map.get(&'\0'), None);
    }
}