use async_trait::async_trait;
use anyhow::Result;

use crate::{
    db::DbConnection,
    model::url_entry::{NewUrlEntry, UrlEntry},
};

#[async_trait]
pub trait UrlEntryRepository {
    async fn fetch_all(&self) -> Result<Vec<UrlEntry>>;
    async fn fetch_by_external_id(&self, external_id: &str) -> Result<UrlEntry>;
    async fn insert(&self, url_entry: NewUrlEntry) -> Result<UrlEntry>;
}

pub struct UrlEntryRepositoryImpl {
    conn: DbConnection,
}

impl UrlEntryRepositoryImpl {
    pub fn new(conn: DbConnection) -> Self {
        UrlEntryRepositoryImpl { conn }
    }
}

#[async_trait]
impl UrlEntryRepository for UrlEntryRepositoryImpl {
    async fn fetch_all(&self) -> Result<Vec<UrlEntry>> {
        let entries = sqlx::query_as!(UrlEntry, "SELECT * FROM url")
            .fetch_all(&self.conn.pool)
            .await?;

        Ok(entries)
    }

    async fn fetch_by_external_id(&self, external_id: &str) -> Result<UrlEntry> {
        let entry = sqlx::query_as!(
            UrlEntry,
            "SELECT * FROM url WHERE external_id = $1",
            external_id
        )
        .fetch_one(&self.conn.pool)
        .await?;

        Ok(entry)
    }

    async fn insert(&self, url_entry: NewUrlEntry) -> Result<UrlEntry> {
        let new_entry = sqlx::query_as!(
            UrlEntry,
            "INSERT INTO url (external_id, target_url) VALUES ( $1, $2 )
            RETURNING *",
            &url_entry.external_id,
            &url_entry.target_url
        )
        .fetch_one(&self.conn.pool)
        .await?;

        Ok(new_entry)
    }
}
