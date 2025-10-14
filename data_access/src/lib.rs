use mysql::{Opts, Pool, PooledConn};

pub fn get_connection() -> PooledConn {
    let url =
        std::env::var("DATABASE_URL").expect("Couldn't get secret key from cargo environment");
    let pool = Pool::new(Opts::from_url(&url).unwrap()).expect("Failed to create pool");

    pool.get_conn().expect("Failed to get connection")
}

pub fn get_connection_safe() -> Result<PooledConn, mysql::Error> {
    let url =
        std::env::var("DATABASE_URL").expect("Couldn't get secret key from cargo environment");
    let pool = Pool::new(Opts::from_url(&url).unwrap())?;
    let conn = pool.get_conn()?;
    Ok(conn)
}
