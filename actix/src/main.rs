use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, Actix!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new().service(index)
    })
    .bind("127.0.0.1:8080")?;

    println!("\nServer is running at http://127.0.0.1:8080");

    server.run().await
}
