use shortly_data::repository::url_repository::UrlEntryRepository;
use std::sync::Arc;

use crate::model::url_entry::UrlEntry;

pub struct AllEntriesUseCase<Repository: UrlEntryRepository> {
    repository: Arc<Repository>,
}

impl<Repository: UrlEntryRepository> AllEntriesUseCase<Repository> {
    pub fn new(repository: Repository) -> Self {
        AllEntriesUseCase {
            repository: Arc::new(repository),
        }
    }

    pub async fn execute(&self) -> Vec<UrlEntry> {
        self.repository
            .fetch_all()
            .await
            .unwrap()
            .into_iter()
            .map(|entry| UrlEntry::from(entry))
            .collect()
    }
}
