use diesel_async::pg::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenvy::dotenv;
use std::env;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

// Note: This must be 'async' because bb8::Pool::builder().build() is async
pub async fn establish_connection_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    
    bb8::Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create async pool")
}
