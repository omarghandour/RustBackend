use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello Friend")
}
#[post("/")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello  Friend")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home).service(create))
        .bind("0.0.0.0.1:8080")?
        .run()
        .await
}
