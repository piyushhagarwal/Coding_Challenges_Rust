use std::env; // Module to get the command line arguments
use std::fs::File; // Module to work with files
use std::io::Read; // Module to read from files

fn count_bytes(contents: &str) -> usize {
    // Give the number of bytes in the file
    contents.len()
}

fn count_words(contents: &str) -> usize {
    // Give the number of words in the file
    let total_words = contents.split_whitespace().count();
    total_words
}

fn count_lines(contents: &str) -> usize {
    // Give the number of lines in the file
    let total_lines = contents.lines().count();
    total_lines
}

fn main() {
    let args : Vec<String> = env::args().collect(); // collect the command line arguments into a vector

    // When you just run the program without any arguments, path of the program is stored in the first element of the vector
    // cargo run - ["target/debug/ccwc"]

    // When you run the program with arguments, the arguments are stored in the vector 
    // cargo run 1 2 3 - ["target/debug/ccwc", "1", "2", "3"]

    // Print the vector
    // println!("{:?}", args); 

    // Length of the vector
    let args_len = args.len() - 1; // subtract 1 to exclude the path of the program

    if args_len == 2{
        // Get the method from the arguments
        let method = &args[1];

        // Get the file name from the arguments
        let file_name = &args[2];

        // Open the file
        let mut file = File::open(file_name).expect("File not found");

        // Read the file
        // Read from the file
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");

        if method == "-c" {
            println!("  {} {}", count_bytes(&contents), file_name);
        }
        else if method == "-w" {
            println!("  {} {}", count_words(&contents), file_name);
        } 
        else if method == "-l" {
            println!("  {} {}", count_lines(&contents), file_name);
        }
        else {
            eprintln!("Usage: ccwc <method> <file_name>");
            std::process::exit(1);
        }

    }

    else if args_len == 1 {
        // Get the file name from the arguments
        let file_name = &args[1];

        // Open the file
        let mut file = File::open(file_name).expect("File not found");

        // Read the file
        // Read from the file
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");

        // Give the number of lines, words, and bytes in the file
        println!("  {} {} {} {}", count_lines(&contents), count_words(&contents), count_bytes(&contents), file_name);
    }

    else{
        eprintln!("Usage: ccwc <method> <file_name>");
        std::process::exit(1);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bytes() {
        assert_eq!(count_bytes("Hello, world!"), 13);
        assert_eq!(count_bytes(""), 0);
        assert_eq!(count_bytes("12345"), 5);
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("Hello, world!"), 2);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("This is a test."), 4);
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines("Hello\nworld"), 2);
        assert_eq!(count_lines(""), 0);
        assert_eq!(count_lines("Line 1\nLine 2\nLine 3"), 3);
    }
}