use log::{info, warn};
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::thread::sleep;
use std::time::Duration;

pub struct ElizaHead {

}

impl ElizaHead {
    pub fn new() -> ElizaHead {
        info!("ElizaHead server is starting...");
        ElizaHead {

        }
    }

    pub fn start(&self) {

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        info!("started.");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Handle the connection in a separate thread
                    std::thread::spawn(|| {
                        ElizaHead::handle_connection(stream);
                    });
                }
                Err(e) => {
                    println!("Error accepting connection: {}", e);
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        info!("Received connect!");
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(_) => {
                // Verarbeite die empfangene Nachricht
                let received_message = String::from_utf8_lossy(&buffer);
                println!("Received message: {}", received_message);

                // Sende eine Antwort an den Client
                let response = "Hello from server!";
                stream.write_all(response.as_bytes()).unwrap();

                sleep(Duration::from_secs(1));


                stream.shutdown(std::net::Shutdown::Both).unwrap();
            }
            Err(e) => {
                println!("Error reading from client: {}", e);
            }
        }
    }
}