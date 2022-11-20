use anyhow::Result;
use shortly_data::repository::url_repository::UrlEntryRepository;
use std::sync::Arc;

use crate::model::url_entry::UrlEntry;

pub struct UrlEntryUseCase<Repository: UrlEntryRepository> {
    repository: Arc<Repository>,
}

impl<Repository: UrlEntryRepository> UrlEntryUseCase<Repository> {
    pub fn new(repository: Repository) -> Self {
        UrlEntryUseCase {
            repository: Arc::new(repository),
        }
    }

    pub async fn execute(&self, external_id: &str) -> Result<UrlEntry> {
        let entry = self.repository.fetch_by_external_id(external_id).await?;

        Ok(UrlEntry::from(entry))
    }
}
