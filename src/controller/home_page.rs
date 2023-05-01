use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::future::Future;
use actix_web::HttpResponse;
use crate::model::database::{select_all_from_table, selecting};
use futures::future;

pub async fn get_all_posts()-> HttpResponse
{

let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template)
        .unwrap();


    let x= selecting().await.expect("adssad");

    let all_posts_to_front_end:String=select_all_from_table().await.expect("adssad");
    let mut data =HashMap::new();
    data.insert("1",x);
data.insert("2",all_posts_to_front_end);
    let html = handlebars.render("index", &data).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}