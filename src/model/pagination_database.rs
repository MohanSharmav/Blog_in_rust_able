use std::fmt::Error;
// use actix_web::{get, web, HttpResponse, Result};
// use serde::Deserialize;
//
// #[derive(Deserialize)]
// struct PaginationParams {
//     page: Option<usize>,
//     limit: Option<usize>,
// }
//
// #[get("/users")]
// async fn get_users(pagination: web::Query<PaginationParams>) -> Result<HttpResponse> {
//     let page = pagination.page.unwrap_or(1); // Default page is 1
//     let limit = pagination.limit.unwrap_or(10); // Default limit is 10
//
//     // Fetch users from the database using the page and limit parameters
//     let users = fetch_users(page, limit).await?;
//
//     // Return the paginated response
//     let response = PaginatedResponse {
//         page,
//         limit,
//         data: users,
//     };
//     Ok(HttpResponse::Ok().json(response))
// }
//
// struct PaginatedResponse<T> {
//     page: usize,
//     limit: usize,
//     data: Vec<T>,
// }
use actix_web::{web, HttpResponse, Result, ResponseError};
use serde::Deserialize;
use crate::model::database::{posts, select_posts};

#[derive(Deserialize)]
struct PaginationParams {
    page: Option<i32>,
    per_page: Option<i32>,
}

fn paginate<T>(items: Vec<T>, page: i32, per_page: i32) -> Vec<T> {
    let start_index = (page - 1) * per_page;
    let end_index = start_index + per_page;
    items.into_iter().skip(start_index as usize).take(per_page as usize).collect()
}

pub async fn get_users() -> Result<(),Error> {
  //  let users = vec!["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Heidi", "Ivan", "Judy"];
    // let page = params.page.unwrap_or(1);
    // let per_page = params.per_page.unwrap_or(5);
   let  page =1;
    let per_page =1;
    let posts_pagination:Vec<posts>= select_posts().await.expect("maosdso");
    let paginated_users = paginate(posts_pagination, page, per_page);


    println!("{:?}", paginated_users);
    // let response = HttpResponse::Ok().json(paginated_users);
    Ok(())
  //  Ok(())
}


