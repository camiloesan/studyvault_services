use mysql::{Pool, PooledConn};

pub fn get_connection() -> PooledConn {
    let url = "mysql://root:123456@mysql:3306/study_vault";
    // let url = "mysql://root:123456@127.0.0.1:6609/study_vault";
    let pool = Pool::new(url).expect("connection failed");
    let conn = pool.get_conn().expect("connection failed");
    conn
}
