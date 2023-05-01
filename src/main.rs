mod model;
mod controller;

use std::env::Args;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::io::Read;
use std::path::Path;
use sqlx::postgres::PgPoolOptions;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result, web};
use actix_web::http::StatusCode;
use tokio::select;
use warp::reply::with_status;
use controller::home_page::get_all_posts;
use model::database::selecting;
use warp::{get, Rejection, Reply};

async fn index(req: HttpRequest)-> Result<NamedFile>{
     let path= Path::new("templates/index.hbs");
     Ok(NamedFile::open(path)?)
}

#[tokio::main]
async fn main() -> Result<()>{

//test start
     get_all_posts().await;


selecting().await.expect("TODO: panic message");
     
     //test end 
     HttpServer::new(|| {
          App::new()
              .service(web::resource("/").to(index))
              .service(web::resource("/hi").to(index))
              .service(web::resource("/hello").to(get_all_posts))
     })
         .bind("127.0.0.1:8080")?
         .run().await.expect("TODO: panic message");
     Ok(())
}
