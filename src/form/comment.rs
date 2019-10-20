#[derive(Debug, FromForm)]
pub struct FormInput {
    // pub id: i32,
    pub username: String,
    pub email: String,
    pub text: String,
}
