use std::fs;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: u16,
}

fn load_config() -> Config {
    let data = fs::read_to_string("config.toml").unwrap();
    toml::from_str(&data).unwrap()
}

fn main() {
    let config = load_config();
    let address = format!("{}:{}", config.ip, config.port);

    let mut stream = TcpStream::connect(&address).unwrap();
    let mut read_stream = stream.try_clone().unwrap();

    println!("Please enter a nickname:");
    let mut nick = String::new();
    io::stdin().read_line(&mut nick).unwrap();

    stream.write_all(nick.trim().as_bytes()).unwrap();

    thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            let bytes = read_stream.read(&mut buffer).unwrap();
            if bytes == 0 {
                break;
            }
            print!("{}", String::from_utf8_lossy(&buffer[..bytes]));
        }
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stream.write_all(input.as_bytes()).unwrap();
    }
}
