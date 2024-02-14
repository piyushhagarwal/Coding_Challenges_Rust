use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args : Vec<String> = env::args().collect(); // collect the command line arguments into a vector

    // When you just run the program without any arguments, path of the program is stored in the first element of the vector
    // cargo run - ["target/debug/ccwc"]

    // When you run the program with arguments, the arguments are stored in the vector 
    // cargo run 1 2 3 - ["target/debug/ccwc", "1", "2", "3"]

    // Print the vector
    // println!("{:?}", args); 

    if args.len() != 3 {
        eprintln!("Usage: ccwc <method> <file_name>");
        std::process::exit(1);
    }

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

    // Give the number of bytes in the file
    println!("  {} {}", contents.len(), file_name);
}
