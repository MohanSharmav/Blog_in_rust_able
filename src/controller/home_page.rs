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

    let all_posts_to_front_end=selecting().await.expect("adssad");



    let html = handlebars.render("data", &all_posts_to_front_end).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}