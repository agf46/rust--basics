use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs; 

// Function to just establish a connection
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


// in the function below, make the stream param mutable since reading the stream might change the internal data in there 
fn handle_connection(mut stream: TcpStream) {
    // To read the stream, we actually need to hold the data somewhere. Which is why I created a buffer to do so. Its 1024 bits and large enough in size. 
    // Keeping it simple if I wanted to read a larger stream of data 
    let mut buffer = [0; 1024]; 

    stream.read(&mut buffer).unwrap();


    // Send a response only if request is GET 
    // Transform the get request to bytes with the "b" string here since we're reading raw bytes into the buffer. 
    let get = b"GET / HTTP/1.1\r\n"; 

    if buffer.starts_with(get) {
    // the html file is included in the repo
        let contents = fs::read_to_string("hello.html").unwrap();
        //write a response with a 200 OK 
        let response = format! (
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        // Wait till all bytes are written
        stream.flush().unwrap();
} else {
    // handle requests that are not a GET 

    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let contents = fs::read_to_string("404.html").unwrap();

    let response = format! ("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    // Wait till all bytes are written
    stream.flush().unwrap();

    }
}

