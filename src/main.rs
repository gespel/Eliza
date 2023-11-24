mod elizaserver;
mod elizahead;

use std::env;
use log::{info, warn};
use elizahead::eliza_head::ElizaHead;
use elizaserver::eliza_server::ElizaServer;

fn main() {
    info!("Eliza version v0.01");
    let args: Vec<String> = env::args().collect();
    if args[1] == "head" {
        let h: ElizaHead = ElizaHead::new();
    }
    else if args[1] == "server" {
        let s: ElizaServer = ElizaServer::new();
        s.start_comm_thread().unwrap();
        let err = s.start_server();
    }
    else {

    }
    println!("{}", args[0]);
}
