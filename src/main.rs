use std::net;
use std::io::{self, prelude::*};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = "seashells.io client written in Rust")]
struct Cli {

    #[arg(short, long, default_value = "seashells.io")]
    url: String,
    
    #[arg(short, long, default_value = "1337")]
    port: String,

    #[arg(short, long, default_value = "false")]
    quiet: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let target = format!("{}:{}", cli.url, cli.port);

    let mut stream = net::TcpStream::connect(target)?;
    let mut buffer: Vec<u8> = vec![0; 64];

    stream.read(&mut buffer)?;
    let string = String::from_utf8(buffer)
        .expect("read into buffer");
    print!("{string}");

    let stdin = io::stdin();
    loop {
        let mut input_buffer = String::new();
        stdin.read_line(&mut input_buffer)?;
        if !cli.quiet {
            print!("{input_buffer}");
        }
        stream.write(&mut input_buffer.as_bytes())?;
    }
    Ok(())
}
