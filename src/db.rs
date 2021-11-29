use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager,PoolError,Pool};
use std::env;


pub type PgPool = Pool <ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection()-> PgPool{
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    init_pool(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}