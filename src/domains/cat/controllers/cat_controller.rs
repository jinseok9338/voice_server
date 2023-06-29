use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;

use crate::{
    domains::cat::{
        dto::{cat_dto::Cat, new_cat::NewCat},
        services::cat_service::CatService,
    },
    Db,
};

#[get("/cats")]
async fn get_cats() -> impl Responder {
    let mut conn = Db::connect_to_db();
    let mut service = CatService::new(&mut conn);
    let cats = service.read_cats();
    HttpResponse::Ok().json(cats)
}

#[post("/cats")]
async fn create_cat(cat: web::Json<NewCat>) -> impl Responder {
    let mut conn = Db::connect_to_db();
    let mut service = CatService::new(&mut conn);
    let created_cat = service.create_cat(&cat);
    HttpResponse::Created().json(created_cat)
}

#[get("/cats/{id}")]
async fn get_cat(path: web::Path<i32>) -> impl Responder {
    let mut conn = Db::connect_to_db();
    let id = path.into_inner();
    let mut service = CatService::new(&mut conn);
    if let Some(cat) = service.read_one_cat(id) {
        HttpResponse::Ok().json(cat)
    } else {
        HttpResponse::NotFound().json(json!({ "message": "Cat not found" }))
    }
}

#[put("/cats/{id}")]
async fn update_cat(path: web::Path<i32>, cat: web::Json<NewCat>) -> impl Responder {
    let mut conn = Db::connect_to_db();
    let mut service = CatService::new(&mut conn);
    let id = path.into_inner();
    let now = Utc::now().naive_utc();
    let matched = service.read_one_cat(id);
    match matched {
        Some(found_cat) => {
            //chagne the value inside the cat with the newCat value
            let update_cat_input = Cat {
                id: found_cat.id,
                name: cat.name.clone(),
                age: cat.age,
                breed: cat.breed.clone(),
                color: cat.color.clone(),
                weight: cat.weight,
                image: Some(cat.image.clone()),
                // updated_at: now
                updated_at: now,
                created_at: found_cat.created_at,
            };
            let updated_cat: Cat = service.update_cat(&update_cat_input);
            HttpResponse::Ok().json(updated_cat)
        }
        None => HttpResponse::NotFound().json(json!({ "message": "Cat not found" })),
    }
}

#[delete("/cats/{id}")]
async fn delete_cat(path: web::Path<i32>) -> impl Responder {
    let mut conn = Db::connect_to_db();
    let id = path.into_inner();
    let mut service = CatService::new(&mut conn);
    if service.read_one_cat(id).is_some() {
        service.delete_cat(id);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(json!({ "message": "Cat not found" }))
    }
}
