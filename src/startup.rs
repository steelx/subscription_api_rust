use std::net::TcpListener;
use actix_web::{App, HttpRequest, HttpServer, Responder, web};
use actix_web::dev::Server;
use sqlx::{PgPool};
use crate::routes::{health_check, subscribe};

async fn greet(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Hello {}!", &name)
}

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
  let addr = listener.local_addr().unwrap().to_string();

  // Wrap the connection in a smart pointer
  let connection = web::Data::new(connection);
  // Capture `connection` from the surrounding environment
  let server = HttpServer::new(move || {
    App::new()
      .route("/", web::get().to(greet))
      .route("/health_check", web::get().to(health_check))
      .route("/subscriptions", web::post().to(subscribe))
      .app_data(connection.clone())
  })
    .listen(listener)?
    .run();

  println!("Server is running at {}", &addr);

  Ok(server)
}