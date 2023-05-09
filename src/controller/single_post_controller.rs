use actix_web::{HttpResponse, web};
use crate::model::Single_posts_database::query_single_post;

pub async fn get_single_post(path: web::Path<String>) -> HttpResponse
{
    let mut titles=path.into_inner();
    println!("{:?}",titles);

query_single_post(titles.clone()).await;
    println!("⭐ ⭐ ⭐ {:?}",titles);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(titles)
}
