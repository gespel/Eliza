use std::thread;
use std::time::Duration;
use actix_web::{web, App, HttpResponse, HttpServer, get, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct ServerStatus {
    name: String
}

pub struct ElizaServer {

}

impl ElizaServer {
    pub fn new() -> ElizaServer {
        ElizaServer{

        }
    }
    #[actix_web::main]
    pub async fn start_server(self) -> std::io::Result<()> {
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
}