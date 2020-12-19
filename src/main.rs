mod api;

use chrono::DateTime;
use chrono::offset::Utc;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_now() {
  let s: DateTime<Utc> = SystemTime::now().into();
  println!("Running Server: {}", s.format("%d/%m/%Y %T"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  get_now();
  api::run()
  .await
}