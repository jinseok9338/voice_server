// use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
// use chrono::Utc;
// use serde_json::json;
// use crate::errors::base_error_messages::BaseError;

// #[get("/notification")]
// async fn get_notifications() -> impl Responder {
//     let mut conn = Db::connect_to_db();
//     // Rest of the code
// }

// #[post("/notification")]
// async fn create_notification(notification: web::Json<Notification>) -> Result<impl Responder, BaseError> {
//     let mut conn = Db::connect_to_db();
//     // Rest of the code
// }

// #[get("/notification/{id}")]
// async fn get_notification(path: web::Path<i32>) -> Result<impl Responder, BaseError> {
//     let mut conn = Db::connect_to_db();
//     // Rest of the code
// }

// #[put("/notification/{id}")]
// async fn update_notification(path: web::Path<i32>, notification: web::Json<Notification>) -> Result<impl Responder, BaseError> {
//     let mut conn = Db::connect_to_db();
//     // Rest of the code
// }

// #[delete("/notification/{id}")]
// async fn delete_notification(path: web::Path<i32>) ->  Result<impl Responder, BaseError> {
//     // Rest of the code
// }

// Do we need controller for this ?
