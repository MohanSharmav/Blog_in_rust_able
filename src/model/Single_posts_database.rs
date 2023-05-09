use sqlx::{Error, FromRow, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};
use crate::model::database::posts;

pub async fn query_single_post(titles: String) ->Result<(),Error>
{

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");
//
//  let rows=  sqlx::query!(r#"select title,description,name from posts where title =$1"#,titles)
//      .fetch_optional(&pool)
//      .await;
// println!("{:?}",rows);

    // generates

    // impl<'r> FromRow<'r, PgRow> for posts {
    //     fn from_row(row: &'r PgRow) -> Result<Self, Error> {
    //         let title = row.try_get("title")?;
    //         let description = row.try_get("description")?;
    //         let name = row.try_get("name")?;
    //         Ok(posts{ title, description, name})
    //     }
    // }

    let  rows = sqlx::query("SELECT name,title,description FROM posts WHERE title =$1")
        .bind(titles)
        .fetch_all(&pool)
        .await.expect("Unable to");


    // let rows=  sqlx::query!(r#"select title,description,name from posts where title =$1"#,titles)
    //     .fetch_optional(&pool)
    //     .await;
    // println!("{:?}",rows);
    for row in rows {
        let title: String = row.get("title");
        let description: String = row.get("description");
        let name: String = row.get("name");
        println!("⭐ ⭐ ⭐ ⭐ ⭐ ⭐ ⭐ ⭐ ⭐{}", title);


    }

    Ok(())
}