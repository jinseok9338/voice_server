use redis::{Client, RedisError};

pub async fn create_redis_client() -> Result<Client, RedisError> {
    dotenv::dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    Client::open(redis_url)
}
