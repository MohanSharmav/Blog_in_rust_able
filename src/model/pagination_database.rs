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
use askama::Html;
use serde::Deserialize;
use crate::model::database::{posts, select_posts};

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<i32>,
    per_page: Option<i32>,
}


use actix_web::{ App, Error as ActixError, HttpServer};

#[derive(Debug)]
pub struct MyError {
    error: ActixError,
}

impl std::convert::From<ActixError> for MyError {
    fn from(error: ActixError) -> Self {
        Self { error }
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "An error occurred: \"{}\"",
            self.error.to_string()
        ))
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.error.as_response_error().status_code()
    }



}
pub fn paginate<T>(items: Vec<T>, page: i32, per_page: i32) -> Vec<T> {
    let start_index = (page - 1) * per_page;
    let end_index = start_index + per_page;
    items.into_iter().skip(start_index as usize).take(per_page as usize).collect()
}

//pub async fn get_users(params: web::Path<String>) -> Result<HttpResponse,MyError> {
pub async fn get_users(params: web::Query<PaginationParams>  ) -> Result<Vec<posts>,MyError> {

//  let users = vec!["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Heidi", "Ivan", "Judy"];
    // let page = params.page.unwrap_or(1);
    // let per_page = params.per_page.unwrap_or(5);
    // let page = params.page.unwrap_or(1);
    // let per_page = params.per_page.unwrap_or(1);

    // let  page =2;
    // let per_page =1;
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(1);

    let posts_pagination:Vec<posts>= select_posts().await.expect("maosdso");
    let paginated_users = paginate(posts_pagination, page, per_page);


    println!("😀😀😀😀😀😀😀{:?}", paginated_users);
     // let response = HttpResponse::Ok().json(paginated_users);
    Ok(paginated_users)
  //  Ok(())
}

//
// fn getss_users(params: web::Query<PaginationParams>) -> Result<HttpResponse> {
//     let users = vec!["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Heidi", "Ivan", "Judy"];
//     let page = params.page.unwrap_or(1);
//     let per_page = params.per_page.unwrap_or(5);
//     let paginated_users = paginate(users, page, per_page);
//
//
//     let response = HttpResponse::Ok().json(paginated_users);
//     Ok(response)
// }


