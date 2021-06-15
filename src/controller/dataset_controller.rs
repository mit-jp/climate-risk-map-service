use super::AppState;
use actix_web::{get, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all);
    cfg.service(get);
}

#[get("/dataset/{id}")]
async fn get(id: web::Path<String>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    println!("GET: /dataset/{}", id);

    let counties = app_state.database.dataset.by_id(&id).await;

    match counties {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(counties) => HttpResponse::Ok().json(counties),
    }
}

#[get("/dataset/")]
async fn get_all(app_state: web::Data<AppState<'_>>) -> impl Responder {
    println!("GET: /dataset/");

    let counties = app_state.database.dataset.all().await;

    match counties {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(counties) => HttpResponse::Ok().json(counties),
    }
}
