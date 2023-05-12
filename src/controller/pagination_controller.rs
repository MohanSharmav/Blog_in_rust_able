use std::fs;
use actix_web::{HttpResponse, web};
use serde_json::json;
use warp::path;
use crate::model::pagination_database::{get_users, PaginationParams};

pub async fn pagination_display(params: web::Query<PaginationParams> ) ->HttpResponse{
   // let mut titles=path.into_inner();

// query_single_post(titles.clone()).await.expect("TODO: panic message");
    println!("asdsadadsdadadadadadadadadadadadadadad");

    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/pagination_page.hbs").unwrap();
    handlebars
        .register_template_string("pagination_page", &index_template).expect("TODO: panic message");


    let paginators=get_users(params).await.expect("Aasd");

    let html = handlebars.render("pagination_page", &json!({"a":&paginators})).unwrap() ;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}