use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/articles")
        .service(get_articles)
        .service(get_article)
        .service(update_article)
        .service(delete_article);
    cfg.service(scope);
}

// get all articles
#[get("")]
async fn get_articles(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("List all articles")
}

#[get("/{id}")]
async fn get_article(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Get a {} artical", id))
}

#[post("/{id}")]
async fn update_article(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Update a {} artical", id))
}

#[delete("/{id}")]
async fn delete_article(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Delete a {} artical", id))
}
