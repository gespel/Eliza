use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use log::{error, info};
use crate::elizaserver::config_setup::ServerConfig;

pub(crate) struct ComThread {
    ip: String,
    port: i16
}

impl ComThread {
    pub(crate) fn new(x: &ServerConfig) -> ComThread {
        ComThread {
            ip: x.head_ip.clone(),
            port: x.head_port.clone(),
        }
    }

    pub(crate) fn run(&self) {
        let ip = self.ip.clone();
        let port = self.port.clone();
        thread::spawn(move || {
            ComThread::worker_request(ip, port);
        });
    }

    fn worker_request(ip: String, port: i16) {
        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(mut stream) => {
                info!("CONNECTED TO HEAD");
                stream.write("hi".as_bytes()).unwrap();
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).expect("Error reading from server");

                // Verarbeite die empfangene Antwort
                let received_message = String::from_utf8_lossy(&buffer);
                println!("Received message from server: {}", received_message);
                sleep(Duration::from_secs(1));
                ComThread::worker_request(ip, port);
            }
            Err(_) => {
                error!("Error while connecting to head server!! Trying to reconnect in 5 seconds...");
                sleep(Duration::from_secs(5));
                ComThread::worker_request(ip, port);
            }
        }
    }
}