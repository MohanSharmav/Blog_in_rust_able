use std::fs;
use actix_web::{HttpResponse, web};
use serde_json::json;
use warp::path;
use crate::model::pagination_database::{ pagination_logic, PaginationParams};



pub async  fn  get_count_of_posts(x:i32) -> HttpResponse {
    println!("{:?}",x);
    let v=x as i32;
    //  static mut n: i32 = v;
    let total_pages_count= x  as i32;
    println!("😊😊😊😊😊😊{:?}",total_pages_count);



    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut all_posts = Vec::new();


    let rows = sqlx::query("SELECT title,description,name FROM posts")
        .fetch_all(&pool)
        .await?;
    for row in rows {
        let title: String = row.get("title");
        let description: String = row.get("description");
        let name: String = row.get("name");
        let all_posts_string= title+" " +&*description +" "+ &*name;
        // let all_posts_string=format!(title, description, name);
        //   let all_posts_json = posts { title: title.to_string(), description: description.to_string(), name: name.to_string() };
        all_posts.push(all_posts_string);
    }


    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/pagination_page.hbs").unwrap();
    handlebars
        .register_template_string("pagination_page", &index_template).expect("TODO: panic message");

    println!("😊😊😊😊😊😊ijhijijijij 😊😊😊😊😊😊 {:?}",total_pages_count);
    let html = handlebars.render("pagination_page", &json!({"bb":&total_pages_count})).unwrap() ;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}
//
pub async fn pagination_display(params: web::Query<PaginationParams> ) ->HttpResponse{
   // let mut titles=path.into_inner();

    get_count_of_posts().await.expect("asdasd");
// query_single_post(titles.clone()).await.expect("TODO: panic message");
    println!("asdsadadsdadadadadadadadadadadadadadad");

    let mut handlebars= handlebars::Handlebars::new();
    let index_template = fs::read_to_string("templates/pagination_page.hbs").unwrap();
    handlebars
        .register_template_string("pagination_page", &index_template).expect("TODO: panic message");


    let paginators= pagination_logic(params).await.expect("Aasd");


    // let pagination_count:i32= get_count_of_posts().await;

//    println!("s😊😊😊😊😊😊{:?}", pagination_count);

    let html = handlebars.render("pagination_page", &json!({"a":&paginators})).unwrap() ;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}