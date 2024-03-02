use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn write_header_to_file(prefix_table: &HashMap<char, String>, output_file_name: &str){
    let mut file = File::create(output_file_name).expect("Error creating file");
    file.write_all(b"PREFIX TABLE:\n").expect("Error writing to file");
    for (key, value) in prefix_table {
        write!(file, "{}: {}\n", key, value).expect("Error writing to file");
    }
    file.write_all(b"\n").expect("Error writing to file");
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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