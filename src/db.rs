use diesel::mysql::MysqlConnection;
use std::env;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub fn establish_connection() -> Pool<ConnectionManager<MysqlConnection>> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url.to_owned());
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    pool.clone().get().expect(&format!("Error connecting to {}", database_url));

    return pool;
}