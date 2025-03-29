use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
 }

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/", web::get().to(hello_world))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
