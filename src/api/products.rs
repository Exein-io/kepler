use actix_web::{web, HttpResponse};
use std::collections::HashMap;

use super::{
    error::ApplicationError,
    utils::{
        bad_request_body, handle_blocking_error, handle_database_error, internal_server_error,
        ok_to_json,
    },
    ApplicationContext,
};

pub async fn all(ctx: web::Data<ApplicationContext>) -> Result<HttpResponse, ApplicationError> {
    web::block(move || {
        ctx.get_database()
            .map_err(handle_database_error)?
            .get_products()
            .map_err(internal_server_error)
    })
    .await
    .map_err(handle_blocking_error)?
    .map(ok_to_json)
}

pub async fn by_vendor(
    ctx: web::Data<ApplicationContext>,
) -> Result<HttpResponse, ApplicationError> {
    let products = web::block(move || {
        ctx.get_database()
            .map_err(handle_database_error)?
            .get_products()
            .map_err(internal_server_error)
    })
    .await
    .map_err(handle_blocking_error)??;

    let mut grouped: HashMap<String, Vec<String>> = HashMap::new();

    for prod in products {
        if let Some(group) = grouped.get_mut(&prod.vendor) {
            group.push(prod.product.clone());
        } else {
            grouped.insert(prod.vendor.clone(), vec![prod.product.clone()]);
        }
    }

    Ok(HttpResponse::Ok().json(grouped))
}

pub async fn search(
    query: web::Path<String>,
    ctx: web::Data<ApplicationContext>,
) -> Result<HttpResponse, ApplicationError> {
    web::block(move || {
        ctx.get_database()
            .map_err(handle_database_error)?
            .search_products(query.as_str())
            .map_err(bad_request_body)
    })
    .await
    .map_err(handle_blocking_error)?
    .map(ok_to_json)
}
