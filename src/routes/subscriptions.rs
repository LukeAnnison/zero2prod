use sqlx::PgPool;
use actix_web::{HttpResponse, web};
use chrono::Utc;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let requst_id = uuid::Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %requst_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );

    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Storing new subscriber in the database");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "Request_id {} - successfully added '{}' '{}' as a new subscriber",
                requst_id,
                form.email,
                form.name,
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "Request_id {} - failed to add '{}' '{}' as a new subscriber: {}",
                requst_id,
                form.email,
                form.name,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}


