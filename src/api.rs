use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}")]
pub async fn process(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
  format!("{}, {}", name, id)
}

pub async fn api() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(process)).bind("127.0.0.1:8000")?
  .run()
  .await
}