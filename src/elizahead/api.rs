use std::thread;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use crate::elizahead::job_structure_parser::JobStructureParser;


pub struct HeadAPI {

}
impl HeadAPI {
    pub(crate) fn new() -> HeadAPI {
        let jp = JobStructureParser::new();
        jp.parse_jobs();

        HeadAPI{}

    }
    pub(crate) fn run(&self) {
        thread::spawn(move || {
            HeadAPI::run_server();
        });
    }

    #[actix_web::main]
    pub async fn run_server() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(Self::hello))
                .route("/jobs", web::get().to(Self::handle_jobs))
        }).bind(("0.0.0.0", 3333))?
            .run()
            .await
    }

    async fn hello() -> impl Responder {
        HttpResponse::Ok()
            .body("Hi!")
    }

    async fn handle_jobs() -> impl Responder {
        HttpResponse::Ok().body("asdasd")
    }

}