use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, Debug, Deserialize)]
struct ReqInfo {
    name: String,
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
struct RepInfo {
    message: String,
}

async fn poster(info: web::Json<ReqInfo>) -> impl Responder {
    let msg = format!("Welcome {}!", info.name);
    let response = RepInfo { message: msg };

    HttpResponse::Ok().json(response)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("From actix webserver: Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/", web::get().to(hello_world))
            .route("/", web::post().to(poster))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{http::header::ContentType, test, App};

    #[actix_web::test]
    async fn test_hello_world() {
        let app = test::init_service(App::new().route("/", web::get().to(hello_world))).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_poster() {
        let app = test::init_service(App::new().route("/", web::post().to(poster))).await;
        let rq = ReqInfo {
            name: "Alice".to_string(),
        };

        let expected_response = RepInfo {
            message: "Welcome Alice!".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(rq)
            .to_request();

        let resp = test::call_service(&app, req).await;
        let body = test::read_body(resp).await;
        let received_data: RepInfo = serde_json::from_slice(&body).unwrap();
        assert_eq!(received_data, expected_response);

    }
}
