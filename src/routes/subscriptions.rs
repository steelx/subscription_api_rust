use actix_web::{HttpResponse, Responder, web};

#[derive(serde::Deserialize)]
pub struct FormData {
  name: String,
  email: String
}

/// Extract form data using serde.
/// This handler get called only if content type is *x-www-form-urlencoded*
/// and content of the request could be deserialized to a `FormData` struct
pub async fn subscriptions(_form: web::Form<FormData>) -> impl Responder {
  HttpResponse::Ok().finish()// return 200 ok; for now
}
