use anyhow::Result;
use shortly_data::repository::url_repository::UrlEntryRepository;
use std::sync::Arc;

use crate::mapper::url_entry_mapper::NewUrlEntryMapper;
use crate::model::url_entry::{NewUrlEntry, UrlEntry};

pub struct CreateUrlEntryUseCase<Repository: UrlEntryRepository, Mapper: NewUrlEntryMapper> {
    repository: Arc<Repository>,
    mapper: Mapper,
}

impl<Repository: UrlEntryRepository, Mapper: NewUrlEntryMapper>
    CreateUrlEntryUseCase<Repository, Mapper>
{
    pub fn new(repository: Repository, mapper: Mapper) -> Self {
        CreateUrlEntryUseCase {
            repository: Arc::new(repository),
            mapper: mapper,
        }
    }

    pub async fn execute(&self, new_entry: NewUrlEntry) -> Result<UrlEntry> {
        let data = self.mapper.map(new_entry);
        let new_entry_data = self.repository.insert(data).await?;

        Ok(UrlEntry::from(new_entry_data))
    }
}
