use shortly_data::repository::url_repository::UrlEntryRepository;
use std::sync::Arc;
use anyhow::Result;

use crate::model::url_entry::UrlEntry;

pub struct AllUrlEntriesUseCase<Repository: UrlEntryRepository> {
    repository: Arc<Repository>,
}

impl<Repository: UrlEntryRepository> AllUrlEntriesUseCase<Repository> {
    pub fn new(repository: Repository) -> Self {
        AllUrlEntriesUseCase {
            repository: Arc::new(repository),
        }
    }

    pub async fn execute(&self) -> Result<Vec<UrlEntry>> {
        let entries = self.repository
            .fetch_all()
            .await?
            .into_iter()
            .map(|entry| UrlEntry::from(entry))
            .collect();
        
        Ok(entries)
    }
}
