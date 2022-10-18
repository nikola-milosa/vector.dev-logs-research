mod log_types;

use std::env;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use fake::{Fake, Faker};
use log_types::TransportLog;
use serde_json;

#[get("/logs/{cursor}")]
async fn nonempty(cursor: web::Path<String>) -> impl Responder {
    let logs = create_logs(Some(cursor.into_inner()));

    let response = serde_json::to_string(&logs).unwrap();

    HttpResponse::Ok().body(response)
}

#[get("/logs")]
async fn empty() -> impl Responder {
    let logs = create_logs(None);

    let response = serde_json::to_string(&logs).unwrap();

    HttpResponse::Ok().body(response)
}

fn create_logs(cursor: Option<String>) -> Vec<TransportLog> {
    let num_logs = rand::random::<u8>() % 10 + 1;
    let mut logs = Vec::new();

    for _ in 0..num_logs {
        let log: TransportLog = Faker.fake();

        if let Some(ref cursor) = cursor {
            let log = TransportLog {
                __cursor: cursor.clone(),
                ..log
            };
            logs.push(log);
        } else {
            logs.push(log)
        }
    }

    logs
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(init_routes))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}

fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(nonempty);
    cfg.service(empty);
}
