mod api;

// use std::env;
// use async_std::task;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  api::api()
  .await
}