use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use log::{info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ServerConfig {
    pub(crate) head_ip: std::string::String,
    pub(crate) head_port: i16
}

pub(crate) fn setup_config() -> ServerConfig {
    let mut sc: ServerConfig;
    if Path::new("server_config.toml").exists() {
        info!("Config file found! Loading...");
        let mut file = File::open("server_config.toml").expect("Error while opening config file!");
        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string).expect("Error while reading config file!");
        sc = toml::from_str(&toml_string).expect("Error while parsing config file!");
    }
    else {
        warn!("Config file not found! Creating new config now...");
        let mut ip_input = String::new();
        let mut port_input = String::new();
        println!("Enter head ip: ");
        io::stdin()
            .read_line(&mut ip_input)
            .expect("Error while reading from stdin!");
        println!("Enter head port: ");
        io::stdin()
            .read_line(&mut port_input)
            .expect("Error while reading from stdin!");

        ip_input = ip_input.trim_end().parse().unwrap();
        port_input = port_input.trim_end().parse().unwrap();

        println!("entered {}:{}", ip_input, port_input);
        let x = ServerConfig {
            head_ip: ip_input,
            head_port: port_input.parse::<i16>().unwrap(),
        };
        let toml_string = toml::to_string(&x).expect("Error while creating toml string!");
        let mut file = File::create("server_config.toml").expect("Error while creating config file!");
        file.write_all(toml_string.as_bytes()).expect("Error while writing config file!");
        sc = x;
    }
    sc
}