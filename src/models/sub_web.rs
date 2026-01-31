use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SubWeb {
    pub id: i32,
    pub name: String,
    pub url: String,
}
