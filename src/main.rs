use std::net;
use std::io::{self, prelude::*};

fn main() -> io::Result<()> {
    let target = "seashells.io:1337";
    let mut stream = net::TcpStream::connect(target)?;
    let string = String::new();
    let mut buffer: Vec<u8> = vec![0; 64];

    stream.read(&mut buffer)?;
    let string = String::from_utf8(buffer)
        .expect("read into buffer");
    // stream.read_to_string(&mut string)?;
    print!("{}", string);
    let mut stdin = io::stdin();

    let quit = false;
    while !quit {
        let mut input_buffer: Vec<u8> = vec![0; 256];
        stdin.read(&mut input_buffer)?;
        let input_string = String::from_utf8(input_buffer.clone())
            .expect("input buffer");
        // stream.read_to_string(&mut string)?;
        print!("{}", input_string);
        stream.write(&mut input_buffer)?;
    }

    Ok(())
}
