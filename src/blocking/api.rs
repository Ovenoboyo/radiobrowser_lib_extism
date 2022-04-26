use crate::blocking::stationsearchbuilder::StationSearchBuilder;
use crate::blocking::CountrySearchBuilder;
use crate::blocking::LanguageSearchBuilder;
use crate::blocking::TagSearchBuilder;

use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct RadioBrowserAPI {
    api: crate::RadioBrowserAPI,
}

use async_std::task;

impl RadioBrowserAPI {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        task::block_on(async { crate::RadioBrowserAPI::new().await })
            .map(|api| RadioBrowserAPI { api })
    }

    pub fn get_stations(&self) -> StationSearchBuilder {
        StationSearchBuilder::new(self.api.get_stations())
    }

    pub fn get_countries(&self) -> CountrySearchBuilder {
        CountrySearchBuilder::new(self.api.get_countries())
    }

    pub fn get_languages(&self) -> LanguageSearchBuilder {
        LanguageSearchBuilder::new(self.api.get_languages())
    }

    pub fn get_tags(&self) -> TagSearchBuilder {
        TagSearchBuilder::new(self.api.get_tags())
    }

    pub fn send<P: AsRef<str>, Q: DeserializeOwned>(
        &mut self,
        endpoint: P,
        mapjson: HashMap<String, String>,
    ) -> Result<Q, Box<dyn Error>> {
        task::block_on(async { self.api.send(endpoint, mapjson).await })
    }

    pub fn get_servers() -> Result<Vec<String>, Box<dyn Error>> {
        task::block_on(async { crate::RadioBrowserAPI::get_servers().await })
    }
}
