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


#[derive(serde::Deserialize)]
struct FormData {
  name: String,
  email: String
}

/// Extract form data using serde.
/// This handler get called only if content type is *x-www-form-urlencoded*
/// and content of the request could be deserialized to a `FormData` struct
async fn subscriptions(_form: web::Form<FormData>) -> impl Responder {
  HttpResponse::Ok().finish()// return 200 ok; for now
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let addr = listener.local_addr().unwrap().to_string();
  let server = HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(greet))
      .route("/health_check", web::get().to(health_check))
      .route("/subscriptions", web::post().to(subscriptions))
      .route("/{name}", web::get().to(greet))
  })
    .listen(listener)?
    .run();

  println!("Server is running at {}", &addr);

  Ok(server)
}