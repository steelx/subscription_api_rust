use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::{PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
  name: String,
  email: String
}

/// Extract form data using serde.
/// This handler get called only if content type is *x-www-form-urlencoded*
/// and content of the request could be deserialized to a `FormData` struct
pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> impl Responder {
  match sqlx::query!(
    r#"
    INSERT INTO subscriptions(id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(), form.email, form.name, Utc::now()
  )
    .execute(connection.get_ref())
    .await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(e) => {
      println!("Failed to execute the query: {}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}
