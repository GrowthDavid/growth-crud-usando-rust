use actix_web::web;
use crate::handlers::{create_product, get_products, update_product, delete_product};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/products")
            .route(web::post().to(create_product))
            .route(web::get().to(get_products)),
    );
    cfg.service(
        web::resource("/products/{id}")
            .route(web::put().to(update_product))
            .route(web::delete().to(delete_product)),
    );
}
