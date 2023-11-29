use std::fmt::format;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{fs, thread};
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use log::{error, info};
use toml::Value::String;
use crate::elizaserver::config_setup::ServerConfig;

#[derive(Clone)]
pub(crate) struct ComThread {
    ip: std::string::String,
    port: i16,
    node_name: std::string::String
}

impl ComThread {
    pub(crate) fn new(x: &ServerConfig) -> ComThread {
        ComThread {
            ip: x.head_ip.clone(),
            port: x.head_port.clone(),
            node_name: x.node_name.clone()
        }
    }

    pub(crate) fn run(&self) {
        let ct = self.clone();
        thread::spawn(move || {
            ComThread::worker_request(&ct);
        });
    }

    fn worker_request(ct: &ComThread) {
        match TcpStream::connect(format!("{}:{}", ct.ip, ct.port)) {
            Ok(mut stream) => {
                info!("CONNECTED TO HEAD");
                stream.write(format!("hi {}", ct.node_name).as_bytes()).unwrap();
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).expect("Error reading from server");

                let received_message = std::string::String::from_utf8_lossy(&buffer);
                let mut words: Vec<&str> = received_message.split_whitespace().collect();
                match words[0] {
                    "jobs" => {
                        words.remove(0);
                        for job in words {

                            ComThread::handle_received_job(job, &mut stream);
                        }
                    }
                    _ => {}
                }

                println!("Received message from server: {}", received_message);
                sleep(Duration::from_secs(1));
                ComThread::worker_request(ct);
            }
            Err(_) => {
                error!("Error while connecting to head server!! Trying to reconnect in 5 seconds...");
                sleep(Duration::from_secs(5));
                ComThread::worker_request(ct);
            }
        }
    }

    fn handle_received_job(jobname: &str, stream: &mut TcpStream) {
        fs::create_dir(format!("{}", jobname)).expect("Error while creating job base folder!");
        stream.write(format!("request {}", jobname).as_bytes()).expect("Error while sending job request");
        loop {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let msg = std::string::String::from_utf8_lossy(&buffer);
            let msg_split: Vec<&str> = msg.split_whitespace().collect();
            let base = msg_split[0];
            let name = msg_split[1];
            match base {
                "folder" => {
                    fs::create_dir(format!("{}/{}", jobname, name)).expect("Error while creating subfolder");
                }
                "file" => {
                    stream.write("rff".as_bytes()).expect("Error while starting filetransfer");
                    let mut fc = Vec::new();
                    stream.read(&mut fc).unwrap();
                    let mut file = File::create(format!("{}/{}", base, name)).unwrap();
                    file.write_all(&fc).expect("Error while writing to file");
                    stream.write("FINISHED".as_bytes()).expect("TODO: panic message");
                }
                "finishedjob" => {
                    return;
                }
                &_ => {
                    return;
                }
            }
        }
    }
}