use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;

// Establish connection to database and read .env file dotenv
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
