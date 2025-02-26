#![feature(string_from_utf8_lossy_owned)]

use std::fs;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

//const REQUEST: &str = "HTTP/1.1 200 OK\r\nContent-Length: 55\r\nContent-Type: text/html\r\nLast-Modified: Wed, 12 Aug 1998 15:03:50 GMT\r\nAccept-Ranges: bytes\r\nETag: “04f97692cbd1:377”\r\nDate: Thu, 19 Jun 2008 19:29:07 GMT\r\n\r\n<55-character response>";

const HOST: &str = "127.0.0.1";
const PORT: &str = "42069";

fn main() -> io::Result<()> {
    //parse_request(REQUEST);

    let listener = TcpListener::bind(format!("{HOST}:{PORT}"))?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(_) = handle_client(stream) {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Encountered error: {e}");
            }
        }
    }
    Ok(())
}

const CHUNK_SIZE: usize = 8192;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    if let Ok(addr) = stream.peer_addr() {
        println!("Incoming connection: {}\n", addr);
    }

    let mut data: Vec<u8> = Vec::new();
    let mut buf = [0; CHUNK_SIZE];
    loop {
        match stream.read(&mut buf) {
            Ok(n) => {
                data.extend_from_slice(&buf[..n]);
                if n < CHUNK_SIZE {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Encountered error while trying to read TcpStream: {}", e);
            }
        }
    }
    let data = String::from_utf8_lossy_owned(data);
    let [statusline, rest] = data.splitn(2, "\r\n").collect::<Vec<_>>()[..] else {
        panic!("Received invalid request format");
    };
    let [header, body] = rest.split("\r\n\r\n").collect::<Vec<_>>()[..] else {
        panic!("Received invalid request format");
    };

    println!("Status line:\n{statusline}\n");
    println!("Header:\n{header}\n");
    println!("Body:\n{body}\n");
    //assert_eq!(parts.len(), 3);

    let status_line = "HTTP/1.1 200 OK";
    let doc = fs::read_to_string("index.html")?;
    let len = doc.len();
    let response = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{doc}");
    stream.write_all(response.as_bytes())?;

    Ok(())
}
