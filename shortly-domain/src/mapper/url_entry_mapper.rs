use crate::model::url_entry::NewUrlEntry;
use shortly_data::model::url_entry::NewUrlEntry as NewUrlEntryData;

pub trait NewUrlEntryMapper {
    fn map(&self, url_entry: NewUrlEntry) -> NewUrlEntryData;
}

pub struct NewUrlEntryMapperImpl;

impl NewUrlEntryMapperImpl {
    pub fn new() -> Self {
        NewUrlEntryMapperImpl {}
    }
}

impl NewUrlEntryMapper for NewUrlEntryMapperImpl {
    fn map(&self, url_entry: NewUrlEntry) -> NewUrlEntryData {
        NewUrlEntryData {
            external_id: url_entry.external_id,
            target_url: url_entry.target_url,
        }
    }
}
