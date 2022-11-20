use shortly_data::model::url_entry::UrlEntry as UrlEntryData;

#[derive(Debug)]
pub struct UrlEntry {
    pub id: i32,
    pub external_id: String,
    pub target_url: String,
}

impl From<UrlEntryData> for UrlEntry {
    fn from(model: UrlEntryData) -> Self {
        UrlEntry {
            id: model.id,
            external_id: model.external_id,
            target_url: model.target_url,
        }
    }
}

#[derive(Debug)]
pub struct NewUrlEntry {
    pub external_id: String,
    pub target_url: String,
}

impl NewUrlEntry {
    pub fn new(external_id: String, target_url: String) -> Self {
        NewUrlEntry {
            external_id,
            target_url,
        }
    }
}
