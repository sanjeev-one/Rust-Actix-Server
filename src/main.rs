use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    println!("Hello world!");
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hi")]
async fn hello() -> impl Responder {
    println!("Hello there thank you for visiting!");
    HttpResponse::Ok().body("Visitor found!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080/");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}