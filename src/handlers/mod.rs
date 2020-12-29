mod user;

use crate::errors::AppError;
use actix_web::{web, web::ServiceConfig, HttpResponse};
use user::create_user;

type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut ServiceConfig) {
    let health_resource = web::resource("/").route(web::get().to(health));

    let signup = web::resource("/signup").route(web::post().to(create_user));

    config.service(health_resource).service(signup);
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
