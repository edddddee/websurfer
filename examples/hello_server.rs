#![feature(string_from_utf8_lossy_owned)]

use std::fs;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

//const REQUEST: &str = "HTTP/1.1 200 OK\r\nContent-Length: 55\r\nContent-Type: text/html\r\nLast-Modified: Wed, 12 Aug 1998 15:03:50 GMT\r\nAccept-Ranges: bytes\r\nETag: “04f97692cbd1:377”\r\nDate: Thu, 19 Jun 2008 19:29:07 GMT\r\n\r\n<55-character response>";

const HOST: &str = "127.0.0.1";
const PORT: &str = "80";

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
    let [req_status_line, rest] = data.splitn(2, "\r\n").collect::<Vec<_>>()[..] else {
        println!("Unexpected request:\n{}", data);
        panic!();
    };
    let [req_header, req_body] = rest.split("\r\n\r\n").collect::<Vec<_>>()[..] else {
        println!("Unexpected request:\n{}", data);
        panic!();
    };

    println!("Status line:\n{req_status_line}");
    println!("Header:\n{req_header}\n");
    println!("Body:\n{req_body}\n");
    //assert_eq!(parts.len(), 3);

    
    let response: Vec<u8> = if req_status_line.contains("favicon.ico") {
        let res_status_line = "HTTP/1.1 200 OK";
        let body = fs::read("favicon.ico")?;
        let len = body.len();
        let mut response = Vec::from(
            format!("{res_status_line}\r\nContent-Length: {len}\r\n\r\n")
            .as_bytes()
        );
        response.extend(&body);
        response
    } else if req_status_line.contains(" / ") {
        let res_status_line = "HTTP/1.1 200 OK";
        let body = fs::read("index.html")?;
        let len = body.len();
        let mut response = Vec::from(
            format!("{res_status_line}\r\nContent-Length: {len}\r\n\r\n")
            .as_bytes()
        );
        response.extend(&body);
        response
    } else {
        vec![42;42]
    };
    stream.write_all(&response)?;

    Ok(())
}
