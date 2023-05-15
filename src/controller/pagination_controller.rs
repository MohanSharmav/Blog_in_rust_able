use std::fs;
use actix_web::{HttpResponse, web};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Row};
use warp::path;
use crate::model::database::{posts, select_posts};
use crate::model::pagination_database::{ pagination_logic, PaginationParams};
use std::fmt::Debug;
use sqlx::postgres::PgRow;


/*fn setup-connection () -> poll{
    // returns connection
}

pub async  fn  get_count_of_posts () -> i32 {
    // Returns i32 count of all posts in database table
}

pub async  fn  get_posts (limit:i32, offset:i32) -> i32 {
    // Returns only limit number of posts, starting from the offset
}*/

pub async  fn  get_count_of_posts () -> HttpResponse {
    // println!("{:?}",x);
    // let v=x as i32;
    // //  static mut n: i32 = v;
    // let total_pages_count= x  as i32;
    // println!("😊😊😊😊😊😊{:?}",total_pages_count);

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut all_posts = Vec::new();


    let rows = sqlx::query("SELECT COUNT(*) FROM posts")
        .fetch_one(&pool)
        .await
        .unwrap();


    for row in rows {
        let title: String = row.get("title");
        let description: String = row.get("description");
        let name: String = row.get("name");
        let all_posts_string= title+" " +&*description +" "+ &*name;
        // let all_posts_string=format!(title, description, name);
        //   let all_posts_json = posts { title: title.to_string(), description: description.to_string(), name: name.to_string() };
        all_posts.push(all_posts_string);
    }

    let total_posts_count:i32 = all_posts.len() as i32;

    println!("Total posts: {}", total_posts_count);


    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/pagination_page.hbs").unwrap();
    handlebars
        .register_template_string("pagination_page", &index_template).expect("TODO: panic message");

    println!("😊😊😊😊😊😊ijhijijijij 😊😊😊😊😊😊 {:?}",total_posts_count);
    let html = handlebars.render("pagination_page", &json!({"bb":&total_posts_count,"yy":"uuihiuhuihiuhuih"})).unwrap() ;


    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}
//
pub async fn pagination_display(params: web::Query<PaginationParams> ) ->HttpResponse{
   // let mut titles=path.into_inner();

    let mut posts_pagination:Vec<posts>= select_posts().await.expect("maosdso");
let mut total_posts_length:i32= posts_pagination.len() as i32;

    total_posts_length=total_posts_length/3;

    let mut pages_count=Vec::new();
    for i in 0..total_posts_length{
     pages_count.push(i+1 as i32);
    }

    println!("pagesss count{:?}", pages_count);
 //   println!("zzzzzzzzzzzzzz{:?}", total_posts_length);
// query_single_post(titles.clone()).await.expect("TODO: panic message");
 //   println!("asdsadadsdadadadadadadadadadadadadadad");

    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/pagination_page.hbs").unwrap();
    handlebars
        .register_template_string("pagination_page", &index_template).expect("TODO: panic message");


    let paginators= pagination_logic(params).await.expect("Aasd");


    // let pagination_count:i32= get_count_of_posts().await;

//    println!("s😊😊😊😊😊😊{:?}", pagination_count);

    let html = handlebars.render("pagination_page", &json!({"a":&paginators,"tt":&total_posts_length,"pages_count":pages_count})).unwrap() ;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}