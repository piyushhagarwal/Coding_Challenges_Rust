use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read, Write};

fn handle_client(mut client_stream: TcpStream) { 
    println!("Received request from {}", client_stream.peer_addr().unwrap());
    let mut buf_reader = BufReader::new(&mut client_stream); // Create a buffer reader to read the request

    // Connect to the backend server
    let mut backend_stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to backend server");

    // Forward request to backend server
    let mut request = String::new();
    buf_reader
        .read_to_string(&mut request)
        .expect("Failed to read request");

    backend_stream
        .write_all(request.as_bytes())
        .expect("Failed to forward request to backend server");

    // Read response from backend server and forward it to the client
    let mut response = String::new();
    backend_stream
        .read_to_string(&mut response)
        .expect("Failed to read response from backend server");

    client_stream 
        .write_all(response.as_bytes())
        .expect("Failed to forward response to client");

    println!("Request processed successfully");
    
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
