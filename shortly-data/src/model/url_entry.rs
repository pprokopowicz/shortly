use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct UrlEntry {
    pub id: i32,
    pub external_id: String,
    pub target_url: String,
}

#[derive(Debug, FromRow)]
pub struct NewUrlEntry {
    pub external_id: String,
    pub target_url: String,
}
