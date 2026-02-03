use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WordCloud {
    pub id: i32,
    pub text: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Work {
    pub id: i32,
    pub year: String,
    pub description: String,
}
