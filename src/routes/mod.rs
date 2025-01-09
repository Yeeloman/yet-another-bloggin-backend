mod articles;
use actix_web::web;


pub fn init(cfg: &mut web::ServiceConfig) {
    articles::config(cfg);
}