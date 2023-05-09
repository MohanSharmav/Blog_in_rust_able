use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::future::Future;
use actix_web::{HttpResponse, web};
use crate::model::database::{select_all_from_table, selecting};
use futures::future;
use serde_json::json;
use warp::body::json;

pub async fn get_all_posts()-> HttpResponse
{

let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template);

    //test start

    //test end

 //   let x= selecting().await.expect("adssad");

    let home_page=select_all_from_table().await.expect("adssad");

    let all_posts_to_front_end=selecting().await.expect("adssad");


    let html = handlebars.render("index", &json!({"o":&all_posts_to_front_end,"p":&home_page})).unwrap() ;

    //let html = handlebars.render("index", &json!({"o":"onee"})).unwrap() ;
println!("{}", html );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}


