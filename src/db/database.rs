use mysql;

pub fn get_connection() -> mysql::Pool
{
    return mysql::Pool::new("mysql://root:root@172.18.0.2:3306/train").unwrap();
}
