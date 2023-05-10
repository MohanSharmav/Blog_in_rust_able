use actix_web::guard::Post;
use serde::Serialize;
use sqlx::{Error, Pool, Postgres, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};
use serde::Deserialize;
use sqlx::FromRow;

#[derive(Deserialize)]
#[derive(Debug, Clone, PartialEq,Serialize)]
pub struct Foo {
    name: String,
}

#[derive(Deserialize)]
#[derive(Debug, Clone, PartialEq,Serialize)]
pub struct posts{
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) name: String,
}

pub(crate) async fn selecting() ->Result<Vec<String>, ()>{


    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");


let mut vect=Vec::new();
    let  rows = sqlx::query("SELECT name FROM categories")
        .fetch_all(&pool)
        .await.expect("Unable to");

    for row in rows{
        let names: String=row.get("name");

      //  let original_Array =Foo { name: names.to_string() };

        vect.push(names);

    }
// let x=std::mem::replace(&mut Foo,"a");
  //  println!("xxxxx x x x x{:?}",x);
  //  let json=serde_json::to_string(&vect).expect("asdasd");
    Ok(vect)
}




pub async fn select_all_from_table() -> Result<Vec<String>,Error> {

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
    println!("⭐ ⭐ ⭐ ⭐ {:?}", all_posts);
//let all_posts_json=serde_json::to_string(&all_posts).expect("noooooo");
    Ok(all_posts)
}
