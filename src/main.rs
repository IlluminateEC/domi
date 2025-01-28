use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct SerializeTest {
    pub a_field: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    web::Json(SerializeTest {
        a_field: "awawawa :3".into(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
