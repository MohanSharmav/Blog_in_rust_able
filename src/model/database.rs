use actix_web::guard::Post;
use serde::Serialize;
use sqlx::{Error, Pool, Postgres, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};

#[derive(Debug, Clone, PartialEq,Serialize)]
pub struct Foo {
    name: String,
}

#[derive(Debug, Clone, PartialEq,Serialize)]
pub struct posts{
    title: String,
    description: String,
    name: String,
}

pub(crate) async fn selecting() ->Result<String, Error>{


    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    select_all_from_table().await.expect("cant select");

let mut vect: Vec<Foo>=Vec::new();
    let  rows = sqlx::query("SELECT name FROM categories")
        .fetch_all(&pool)
        .await.expect("Unable to");

    for row in rows{
        let names: String=row.get("name");

         let original_Array = Foo { name: names.to_string() };

        vect.push(original_Array);

    }

    println!("Fooooo  finalk is  is {:?}", vect);
    let json=serde_json::to_string(&vect).expect("asdasd");
    println!("aaaaaa {:?}",json);
    Ok(json)
}




pub async fn select_all_from_table() -> Result<String,Error> {

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut all_posts:Vec<posts> = Vec::new();


    let rows = sqlx::query("SELECT title,description,name FROM posts")
          .fetch_all(&pool)
          .await?;
    for row in rows {
        let title: String = row.get("title");
        let description: String = row.get("description");
        let name: String = row.get("name");
        let all_posts_json = posts { title: title.to_string(), description: description.to_string(), name: name.to_string() };
        all_posts.push(all_posts_json);
    }
let all_posts_json=serde_json::to_string(&all_posts).expect("noooooo");
    Ok(all_posts_json)
}