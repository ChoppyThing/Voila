use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub slug: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, FromFormValue, Serialize, Deserialize)]
pub enum Categories {
    A, B, C
}

#[derive(Debug, FromForm)]
pub struct FormInput {
    pub title: String,
    pub post: String,
    pub category: i32,
}
