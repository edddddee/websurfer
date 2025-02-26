use std::io;
use std::io::{Read, BufRead, BufReader};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:42069")?;
    let mut reader = BufReader::new(stream);
    
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => { 
                print!("{line}"); 
                break;
            }
            Ok(_) => {
                print!("{line}");
            }
            Err(e) => {
                eprintln!("Error reading line: {e}");
                break;
            }
        }

    }
    Ok(())
}
