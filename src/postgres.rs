use diesel::prelude::*;
use diesel::sql_types::BigInt;
use diesel::{sql_query, PgConnection, QueryableByName};

#[derive(QueryableByName)]
pub struct Count {
    #[sql_type = "BigInt"]
    pub n: i64,
}

pub fn connection(db_url: &str) -> PgConnection {
    PgConnection::establish(&db_url).unwrap()
}

pub fn late_schedule_count(conn: &PgConnection) -> u64 {
    let count: Count = sql_query(include_str!("sql/late_schedule_count.sql"))
        .get_result(conn)
        .unwrap();
    
    assert!(count.n > -1);
    count.n as u64
}

#[cfg(test)]
mod tests {
    extern crate dotenv;
    use super::*;

    #[test]
    fn test_late_schedule_count() {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = connection(&db_url);

        println!("Number of late schedules: {}", late_schedule_count(&conn));
    }
}
