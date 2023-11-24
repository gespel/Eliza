use std::num::ParseIntError;
use log::{error, info, warn};
use actix_web::{web, App, HttpResponse, HttpServer, get, Responder, Result};
use serde::{Deserialize, Serialize};
use crate::elizaserver::{config_setup, head_communicator};
use crate::elizaserver::config_setup::ServerConfig;

#[derive(Serialize)]
struct ServerStatus {
    name: String
}



pub struct ElizaServer {
    config: ServerConfig,
}

impl ElizaServer {
    pub fn new() -> ElizaServer {
        info!("Starting eliza server now!");
        let config = config_setup::setup_config();

        ElizaServer{
            config
        }
    }



    #[actix_web::main]
    pub async fn start_server(&self) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(Self::hello))
                .route("/status", web::get().to(Self::handle_status))
        }).bind(("0.0.0.0", 1234))?
            .run()
            .await
    }

    async fn hello() -> impl Responder {
        HttpResponse::Ok()
            .body("Hi!")
    }

    async fn handle_status() -> impl Responder {
        let out = ServerStatus {
            name: "Eliza1".to_string(),
        };
        HttpResponse::Ok().json(out)
    }

    pub fn start_comm_thread(&self) -> Result<(), ParseIntError> {
        let hc = head_communicator::ComThread::new();
        hc.run();
        Ok(())
    }
}