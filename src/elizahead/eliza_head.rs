use log::{info, warn};
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::thread::sleep;
use std::time::Duration;
use actix_web::guard::Head;
use crate::elizahead::api;
use crate::elizahead::api::{HeadAPI};

pub struct ElizaHead {
    head_port: i16
}

impl ElizaHead {
    pub fn new() -> ElizaHead {
        info!("ElizaHead server is starting...");
        let ha = HeadAPI::new();
        ha.run();


        ElizaHead {
            head_port: 7878
        }
    }

    pub fn start(&self) {

        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.head_port)).unwrap();
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

        stream.read(&mut buffer).unwrap();

        // Verarbeite die empfangene Nachricht
        let received_message = String::from_utf8_lossy(&buffer);
        println!("Received message: {}", received_message);

        // Sende eine Antwort an den Client
        let response = "jobs test tester stenstester";
        stream.write_all(response.as_bytes()).unwrap();

        sleep(Duration::from_secs(1));


        stream.shutdown(std::net::Shutdown::Both).unwrap();



    }
}