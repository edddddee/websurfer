#![feature(string_from_utf8_lossy_owned)]

use std::fs;
use std::io;
use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;

const HOST: &str = "127.0.0.1";
const PORT: &str = "42069";
const REQUEST: &str = "HTTP/1.1 200 OK\r\nContent-Length: 55\r\nContent-Type: text/html\r\nLast-Modified: Wed, 12 Aug 1998 15:03:50 GMT\r\nAccept-Ranges: bytes\r\nETag: “04f97692cbd1:377”\r\nDate: Thu, 19 Jun 2008 19:29:07 GMT\r\n\r\n<55-character response>";

fn main() -> io::Result<()> {
    //parse_request(REQUEST);

    let listener = TcpListener::bind(format!("{HOST}:{PORT}"))?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Encountered error: {e}");
            }
        }
    }
    Ok(())
}

const CHUNK_SIZE: usize = 8192;

#[derive(Debug, PartialEq)]
enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

#[derive(Debug, PartialEq)]
struct InvalidHttpMethod;

impl FromStr for Method {
    type Err = InvalidHttpMethod;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_ascii_uppercase()[..] {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(InvalidHttpMethod),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Version {
    HTTP_0_9,
    HTTP_1_0,
    HTTP_1_1,
    HTTP_2,
    HTTP_3,
}

#[derive(Debug, PartialEq)]
struct InvalidHttpVersion;

impl FromStr for Version {
    type Err = InvalidHttpVersion;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            r"HTTP/0.9" => Ok(Version::HTTP_0_9),
            r"HTTP/1.0" => Ok(Version::HTTP_1_0),
            r"HTTP/1.1" => Ok(Version::HTTP_1_1),
            r"HTTP/2" => Ok(Version::HTTP_2),
            r"HTTP/3" => Ok(Version::HTTP_3),
            _ => Err(InvalidHttpVersion),

        }
    }
}

#[derive(Debug, PartialEq)]
enum Scheme {
    Standard(CommonSchemes),
    Custom(String),
    Empty,
}

#[derive(Debug, PartialEq)]
enum CommonSchemes {
    HTTP,
    HTTPS,
}

struct Uri {
    scheme: Scheme,
}

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
                if n < CHUNK_SIZE { break; }
            }
            Err(e) => { eprintln!("Encountered error while trying to read TcpStream: {}", e); 
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_parsing() {
        assert_eq!("GET".parse(), Ok(Method::GET));
        assert_eq!("get".parse(), Ok(Method::GET));
        assert_eq!("HEAD".parse(), Ok(Method::HEAD));
        assert_eq!("head".parse(), Ok(Method::HEAD));
        assert_eq!("POST".parse(), Ok(Method::POST));
        assert_eq!("post".parse(), Ok(Method::POST));
        assert_eq!("PUT".parse(), Ok(Method::PUT));
        assert_eq!("put".parse(), Ok(Method::PUT));
        assert_eq!("DELETE".parse(), Ok(Method::DELETE));
        assert_eq!("delete".parse(), Ok(Method::DELETE));
        assert_eq!("CONNECT".parse(), Ok(Method::CONNECT));
        assert_eq!("connect".parse(), Ok(Method::CONNECT));
        assert_eq!("OPTIONS".parse(), Ok(Method::OPTIONS));
        assert_eq!("options".parse(), Ok(Method::OPTIONS));
        assert_eq!("TRACE".parse(), Ok(Method::TRACE));
        assert_eq!("trace".parse(), Ok(Method::TRACE));
        assert_eq!("PATCH".parse(), Ok(Method::PATCH));
        assert_eq!("patch".parse(), Ok(Method::PATCH));

        assert_eq!("gets".parse::<Method>(), Err(InvalidHttpMethod));
    }

    #[test]
    fn test_http_version_parsing() {
        assert_eq!("HTTP/0.9".parse(), Ok(Version::HTTP_0_9));
        assert_eq!("HTTP/1.0".parse(), Ok(Version::HTTP_1_0));
        assert_eq!("HTTP/1.1".parse(), Ok(Version::HTTP_1_1));
        assert_eq!("HTTP/2".parse(), Ok(Version::HTTP_2));
        assert_eq!("HTTP/3".parse(), Ok(Version::HTTP_3));

        // Case sensitive!
        assert_eq!("http/0.9".parse::<Version>(), Err(InvalidHttpVersion));

        assert_eq!("".parse::<Version>(), Err(InvalidHttpVersion));
        assert_eq!("blablabla".parse::<Version>(), Err(InvalidHttpVersion));
    }
}
