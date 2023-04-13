// Import the `Router` module from the `router.rs` file.
mod router;
// Import the necessary I/O and networking modules from the standard library.
use std::io::prelude::*;
use std::net::TcpListener;
// Import the `Router` struct from the `router.rs` file.
use router::Router;

// Define the main function that will run the server.
fn main() {
    // Create a new `Router` instance.
    let mut router = Router::new();

    // Add a new route to the router for the "/" path with a handler function that returns a "Hello, world!" response.
    router.add_route("GET", "/", |stream, _| {
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
        // Write the response string to the TCP stream.
        stream.write(response.as_bytes()).unwrap();
        // Flush the TCP stream to ensure that the response is sent immediately.
        stream.flush().unwrap();
    });

    // Create a new TCP listener that listens for incoming connections on port 8080.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // Print a message to the console indicating that the server is now listening.
    println!("Server listening on port 8080");

    // Loop over incoming connections from clients.
    for stream in listener.incoming() {
        // Unwrap the incoming stream and create a mutable reference to it.
        let mut stream = stream.unwrap();
        // Create a new buffer to hold the incoming data from the client.
        let mut buffer = [0; 1024];
        // Read data from the client into the buffer.
        stream.read(&mut buffer).unwrap();

        // Convert the buffer bytes to a UTF-8 string.
        let request = String::from_utf8_lossy(&buffer[..]);

        // Split the request string into lines and collect them into a vector of string slices.
        let request_lines: Vec<&str> = request.lines().collect();
        // Extract the first line of the request (the request line).
        let request_line = request_lines[0];
        // Split the request line into parts (method, path, and HTTP version) and collect them into a vector of string slices.
        let request_parts: Vec<&str> = request_line.split(" ").collect();
        // Extract the HTTP method (e.g. "GET", "POST", etc.) from the request parts.
        let method = request_parts[0];
        // Extract the request path (e.g. "/index.html") from the request parts.
        let path = request_parts[1];

        // Handle the incoming request using the `Router` instance's `handle_request` method.
        router.handle_request(&mut stream, method, path);
    }
}
