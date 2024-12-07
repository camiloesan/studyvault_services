use mysql::{Pool, PooledConn};

pub fn get_connection() -> PooledConn {
    // let url = "mysql://root:123456@mysql:3306/study_vault";
    let url = "mysql://root:123456@127.0.0.1:6609/study_vault";
    let pool = Pool::new(url).expect("Failed to create pool");

    pool.get_conn().expect("Failed to get connection")
}

pub fn get_connection_safe() -> Result<PooledConn, mysql::Error> {
    let url = "mysql://root:123456@127.0.0.1:6609/study_vault";
    let pool = Pool::new(url)?;
    let conn = pool.get_conn()?;
    Ok(conn)
}
