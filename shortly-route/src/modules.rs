use shortly_data::repository::url_repository::{UrlEntryRepository, UrlEntryRepositoryImpl};
use shortly_domain::mapper::url_entry_mapper::{NewUrlEntryMapper, NewUrlEntryMapperImpl};
use shortly_domain::use_case::{
    all_url_entries_use_case::AllUrlEntriesUseCase,
    create_url_entry_use_case::CreateUrlEntryUseCase, url_entry_use_case::UrlEntryUseCase,
};

pub trait Modules {
    type Repository: UrlEntryRepository;
    type Mapper: NewUrlEntryMapper;

    fn all_url_entries_use_case(&self) -> &AllUrlEntriesUseCase<Self::Repository>;
    fn create_url_entry_use_case(&self) -> &CreateUrlEntryUseCase<Self::Repository, Self::Mapper>;
    fn url_entry_use_case(&self) -> &UrlEntryUseCase<Self::Repository>;
}

pub struct ModulesImpl {
    all_url_entries_use_case: AllUrlEntriesUseCase<UrlEntryRepositoryImpl>,
    create_url_entry_use_case: CreateUrlEntryUseCase<UrlEntryRepositoryImpl, NewUrlEntryMapperImpl>,
    url_entry_use_case: UrlEntryUseCase<UrlEntryRepositoryImpl>,
}

impl ModulesImpl {
    pub fn new(
        all_url_entries_use_case: AllUrlEntriesUseCase<UrlEntryRepositoryImpl>,
        create_url_entry_use_case: CreateUrlEntryUseCase<
            UrlEntryRepositoryImpl,
            NewUrlEntryMapperImpl,
        >,
        url_entry_use_case: UrlEntryUseCase<UrlEntryRepositoryImpl>,
    ) -> Self {
        ModulesImpl {
            all_url_entries_use_case,
            create_url_entry_use_case,
            url_entry_use_case,
        }
    }
}

impl Modules for ModulesImpl {
    type Repository = UrlEntryRepositoryImpl;
    type Mapper = NewUrlEntryMapperImpl;

    fn all_url_entries_use_case(&self) -> &AllUrlEntriesUseCase<Self::Repository> {
        &self.all_url_entries_use_case
    }

    fn create_url_entry_use_case(&self) -> &CreateUrlEntryUseCase<Self::Repository, Self::Mapper> {
        &self.create_url_entry_use_case
    }

    fn url_entry_use_case(&self) -> &UrlEntryUseCase<Self::Repository> {
        &self.url_entry_use_case
    }
}
