use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn handle_client(mut client_stream: TcpStream) {
    // println!("Received request from {}", client_stream.peer_addr().unwrap());

    // Set a read timeout of 5 seconds for the client stream
    client_stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("Failed to set read timeout");

    // Connect to the backend server
    let mut backend_stream = TcpStream::connect("127.0.0.1:7878")
        .expect("Failed to connect to backend server");

    // Forward request to backend server
    let mut buf = [0; 1024]; // Create a buffer to store the incoming data
    loop {
        match client_stream.read(&mut buf) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // Connection closed by the client
                    break;
                }

                backend_stream
                    .write_all(&buf[..bytes_read])
                    .expect("Failed to forward request to backend server");
            }
            Err(e) => {
                eprintln!("Error reading from client stream: {}", e);
                break;
            }
        }
    }

    // Read response from backend server and forward it to the client
    let mut response = Vec::new();
    loop {
        let bytes_read = backend_stream.read(&mut buf).expect("Failed to read response from backend server");
    
        if bytes_read == 0 {
            // Connection closed by the backend server
            break;
        }
    
        response.extend_from_slice(&buf[..bytes_read]);
    
        // Check if the backend server has sent the complete response
        if response.ends_with(b"\r\n\r\n") {
            // The response ends with the final blank line, indicating the end of the response
            break;
        }
    }

    // Write the response to the client
    client_stream
        .write_all(&response)
        .expect("Failed to forward response to client");

    // Flush the client stream to ensure that the response is sent
    client_stream
        .flush()
        .expect("Failed to flush client stream");

    // println!("Request processed successfully");
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