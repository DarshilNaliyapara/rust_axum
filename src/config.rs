use diesel_async::pg::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenvy::dotenv;
use std::env;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn  connect_to_database() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    
    bb8::Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create async pool")
}
