use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(_form: web::Form<FormData>) -> HttpResponse {
     HttpResponse::Ok().finish()
}


