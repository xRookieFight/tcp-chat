use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: u16,
}

fn load_config() -> Config {
    let data = fs::read_to_string("config.toml").unwrap();
    toml::from_str(&data).unwrap()
}

fn handle_client(
    mut stream: TcpStream,
    clients: Arc<Mutex<HashMap<String, TcpStream>>>,
    nick: String,
) {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        let text = String::from_utf8_lossy(&buffer[..bytes_read]);
        let message = format!("[{}] {}", nick, text);

        let mut clients = clients.lock().unwrap();
        for client in clients.values_mut() {
            let _ = client.write_all(message.as_bytes());
            println!("{}", message);
        }
    }

    clients.lock().unwrap().remove(&nick);
}

fn main() {
    let config = load_config();
    let address = format!("{}:{}", config.ip, config.port);

    let listener = TcpListener::bind(&address).unwrap();
    println!("Server started on {}", address);

    let clients: Arc<Mutex<HashMap<String, TcpStream>>> =
        Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = [0; 1024];
        let bytes = stream.read(&mut buffer).unwrap();
        let nick = String::from_utf8_lossy(&buffer[..bytes]).trim().to_string();

        println!("{} connected", nick);

        clients
            .lock()
            .unwrap()
            .insert(nick.clone(), stream.try_clone().unwrap());

        let clients_clone = Arc::clone(&clients);

        thread::spawn(move || {
            handle_client(stream, clients_clone, nick);
        });
    }
}
