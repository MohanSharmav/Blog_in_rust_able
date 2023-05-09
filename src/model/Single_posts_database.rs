use sqlx::{Error, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};

pub async fn query_single_post(titles: String) ->Result<(),Error>
{

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

 let rows=  sqlx::query!(r#"select title,description,name from posts where title =$1"#,titles)
     .fetch_optional(&pool)
     .await;
println!("{:?}",rows);
    // let rows = sqlx::query("select title,description,name from posts where title =mysore")
    //     .fetch_all(&pool)
    //     .await;

    // for row in rows {
    //     let title: String = row.get("title");
    //     let description: String = row.get("description");
    //     let name: String = row.get("name");
    //     println!("⭐ ⭐ ⭐ ⭐ ⭐ ⭐ ⭐ ⭐ ⭐{}", title);
    // }

    Ok(())
}