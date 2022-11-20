use anyhow::{anyhow, Result};
use nanoid::nanoid;
use shortly_data::repository::url_repository::UrlEntryRepository;
use std::sync::Arc;
use url::Url;

use crate::mapper::url_entry_mapper::NewUrlEntryMapper;
use crate::model::url_entry::{NewUrlEntry, UrlEntry};

pub struct CreateUrlEntryUseCase<Repository: UrlEntryRepository, Mapper: NewUrlEntryMapper> {
    repository: Arc<Repository>,
    mapper: Mapper,
}

impl<Repository: UrlEntryRepository, Mapper: NewUrlEntryMapper>
    CreateUrlEntryUseCase<Repository, Mapper>
{
    pub fn new(repository: Arc<Repository>, mapper: Mapper) -> Self {
        CreateUrlEntryUseCase {
            repository: repository,
            mapper: mapper,
        }
    }

    pub async fn execute(&self, url: &str) -> Result<UrlEntry> {
        let chars: [char; 62] = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
            'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
            'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        let _ = Url::parse(&url).map_err(|_| anyhow!("Bad data format"))?;

        let new_entry = NewUrlEntry::new(nanoid!(8, &chars), url.to_string());
        let data = self.mapper.map(new_entry);
        let new_entry_data = self.repository.insert(data).await?;

        Ok(UrlEntry::from(new_entry_data))
    }
}
