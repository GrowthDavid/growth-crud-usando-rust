use actix_web::{web, HttpResponse};
use mongodb::{Client, Collection};
use mongodb::bson::{doc, oid::ObjectId};
use crate::models::Product;
use futures_util::stream::TryStreamExt;

pub async fn create_product(product: web::Json<Product>, db: web::Data<Client>) -> HttpResponse {
    let collection: Collection<Product> = db.database("rust_mongo_crud").collection("products");
    let new_product = Product {
        id: None,
        name: product.name.clone(),
        price: product.price,
        quantity: product.quantity,
    };

    let result = collection.insert_one(new_product, None).await;

    match result {
        Ok(inserted) => HttpResponse::Ok().json(inserted.inserted_id),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn get_products(db: web::Data<Client>) -> HttpResponse {
    let collection: Collection<Product> = db.database("rust_mongo_crud").collection("products");
    let mut cursor = collection.find(None, None).await.unwrap();
    let mut products = Vec::new();

    while let Some(product) = cursor.try_next().await.unwrap() {
        products.push(product);
    }

    HttpResponse::Ok().json(products)
}

pub async fn update_product(id: web::Path<String>, product: web::Json<Product>, db: web::Data<Client>) -> HttpResponse {
    let collection: Collection<Product> = db.database("rust_mongo_crud").collection("products");
    let obj_id = ObjectId::parse_str(id.as_str()).unwrap();

    let result = collection.update_one(
        doc! { "_id": obj_id },
        doc! { "$set": { "name": &product.name, "price": product.price, "quantity": product.quantity } },
        None,
    ).await;

    match result {
        Ok(update_result) => HttpResponse::Ok().json(update_result.modified_count),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_product(id: web::Path<String>, db: web::Data<Client>) -> HttpResponse {
    let collection: Collection<Product> = db.database("rust_mongo_crud").collection("products");
    let obj_id = ObjectId::parse_str(id.as_str()).unwrap();

    let result = collection.delete_one(doc! { "_id": obj_id }, None).await;

    match result {
        Ok(delete_result) => HttpResponse::Ok().json(delete_result.deleted_count),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
