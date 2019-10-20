use mysql;

pub fn get_connection() -> mysql::Pool
{
    return mysql::Pool::new("mysql://root:test@localhost:3306/train").unwrap();
}
