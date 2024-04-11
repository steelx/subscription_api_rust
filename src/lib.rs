use std::net::TcpListener;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

async fn greet(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder{
  HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let addr = listener.local_addr().unwrap().to_string();
  let server = HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(greet))
      .route("/health_check", web::get().to(health_check))
      .route("/{name}", web::get().to(greet))
  })
    .listen(listener)?
    .run();

  println!("Server is running at {}", &addr);

  Ok(server)
}