#[warn(unused_imports)]

use std::path::Path;
use std::fs;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{get, middleware, web, App, Error, HttpServer, HttpResponse, Responder};
use actix_multipart::Multipart;

use futures::{StreamExt, TryStreamExt};
use serde::{Serialize, Deserialize};
use json::JsonValue;

const UPLOAD_PATH: &str = "/Users/cobyeastwood/Desktop";

#[derive(Serialize, Deserialize)]
struct File {
  name: String,
  time: u64,
  err: String
}

#[derive(Deserialize)]
pub struct Download {
  name: String,
}

impl File {
  fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() // unwrap is unsafe !#(must remove on production)
  }
}

async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {

  fs::create_dir_all(UPLOAD_PATH)?;

  let mut filename = "".to_string();

  println!("{}", filename);

  while let Ok(Some(mut field)) = payload.try_next().await {
    let content_type = field.content_disposition().unwrap();

    filename = format!("{} - {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(),
    content_type.get_filename().unwrap(),    );

    let filepath = format!("{}/{}", UPLOAD_PATH, sanitize_filename::sanitize(&filename));

    let mut f = web::block(|| std::fs::File::create(filename)).await.unwrap();

    while let Some(chunk) = field.next().await {
      let data = chunk.unwrap();

      f = web::block(move || f.write_all(&data).map(|_|f)).await?;

    }
  }

  let res = &File {
    name: "dummy data".to_string(),
    time: File::now(),
    err: "".to_string()
  };
  Ok(HttpResponse::Ok().json(res))
}

async fn download(info: web::Path<Download>) -> HttpResponse {
  let path = format!("{}/{}", UPLOAD_PATH, info.name.to_string());

  if !Path::new(path.as_str()).exists() {
    return HttpResponse::NotFound().json(&File {
      name: info.name.to_string(),
      time: File::now(),
      err: "file does not exist".to_string()
    })
  }

  let data = fs::read(path).unwrap();

  HttpResponse::Ok().header("Content-Disposition", format!("form-data; filename={}", info.name.to_string())).body(data)
}

pub async fn index_json(body: web::Bytes) -> Result<HttpResponse, Error> {

  let res = json::parse(std::str::from_utf8(&body).unwrap());

  let json_in = match res {
    Ok(v) => { println!("{}", v); v },
    Err(e) => json::object! {"err" => e.to_string()},
  };

  Ok(HttpResponse::Ok()
    .content_type("application/json")
    .body(json_in.dump()))

}

pub async fn run() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
    .wrap(middleware::Logger::default())
    .data(web::JsonConfig::default().limit(4096))
    .service(
      web::scope("/api").route("/files/", web::post().to(upload))
      .route("/files/{name}/", web::get().to(download))
    )
    .service(web::resource("/json").route(web::post().to(index_json)))
  }).bind("127.0.0.1:8000")?
  .run()
  .await
}

// Testing From Source
// https://medium.com/swlh/build-your-first-rest-api-using-rust-language-and-actix-framework-8f827175b30f