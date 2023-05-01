use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::future::Future;
use actix_web::HttpResponse;
use crate::model::database::selecting;
use futures::future;

pub async fn get_all_posts()-> HttpResponse
{
    println!("⭐⭐⭐⭐⭐⭐  Getting all posts");

let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/index.hbs").unwrap();
    handlebars
        .register_template_string("index", &index_template)
        .unwrap();


    let x:String= selecting().await.expect("adssad");
    println!("s s s s s s {:?}",x);
//     let mut vector=Vec::new();
//
//     vector.push(selecting.to_owned());
// //    println!("{}", vector);
  let mut data =HashMap::new();
//     // data.insert("name","boss");
    data.insert("1",x);
//    // data.insert("2",vec);
//     println!("{:?}",data);
    let html = handlebars.render("index", &data).unwrap();

    //test  start
//test end
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}