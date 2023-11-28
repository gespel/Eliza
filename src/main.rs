mod elizaserver;
mod elizahead;

use std::env;
use log::{error, info, warn};
use elizahead::eliza_head::ElizaHead;
use elizaserver::eliza_server::ElizaServer;

fn main() {
    env_logger::init();
    info!("Eliza version v0.01");
    let args: Vec<String> = env::args().collect();
    if args[1] == "head" {
        let h: ElizaHead = ElizaHead::new();
        h.start();
    }
    else if args[1] == "server" {
        let s: ElizaServer = ElizaServer::new();
        s.start_comm_thread().unwrap();
        let err = s.start_server();
    }
    else {
        error!("No argument given!");
    }
}
