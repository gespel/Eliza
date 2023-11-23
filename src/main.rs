mod eliza_server;
mod eliza_head;

use std::env;
use log::{info, warn};
use crate::eliza_head::ElizaHead;
use crate::eliza_server::ElizaServer;

fn main() {
    info!("Eliza version v0.01");
    let args: Vec<String> = env::args().collect();
    if args[1] == "head" {
        let h: ElizaHead = ElizaHead::new();
    }
    else if args[1] == "server" {
        let s: ElizaServer = ElizaServer::new();
        let err = s.start_server();
    }
    else {

    }
    println!("{}", args[0]);
}
