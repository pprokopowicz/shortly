use serde::{Deserialize, Serialize};
use shortly_domain::model::url_entry::UrlEntry as UrlEntryDomain;

#[derive(Serialize)]
pub struct UrlEntry {
    pub target_url: String,
    pub external_id: String,
}

impl From<UrlEntryDomain> for UrlEntry {
    fn from(model: UrlEntryDomain) -> Self {
        UrlEntry {
            target_url: model.target_url,
            external_id: model.external_id,
        }
    }
}

#[derive(Deserialize)]
pub struct NewUrlEntry {
    pub target_url: String,
}
