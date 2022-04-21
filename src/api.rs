use crate::external::post_api;
use crate::ApiConfig;
use crate::ApiCountry;
use crate::CountrySearchBuilder;
use crate::LanguageSearchBuilder;
use crate::StationSearchBuilder;
use crate::TagSearchBuilder;

use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::error::Error;

use rand::seq::SliceRandom;
use rand::thread_rng;

use async_std_resolver::proto::rr::RecordType;
use async_std_resolver::proto::xfer::DnsRequestOptions;
use async_std_resolver::{config, resolver};

#[derive(Clone, Debug)]
pub struct RadioBrowserAPI {
    servers: Vec<String>,
    current: usize,
}

impl RadioBrowserAPI {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(RadioBrowserAPI {
            servers: RadioBrowserAPI::get_servers().await?,
            current: 0,
        })
    }

    pub fn get_current_server(&mut self) -> String {
        if self.servers.len() > 0 {
            self.current = self.current % self.servers.len();
            format!("https://{}", self.servers[self.current])
        } else {
            String::from("https://de1.api.radio-browser.info")
        }
    }

    pub async fn post_api<P: DeserializeOwned, A: AsRef<str>>(
        &mut self,
        endpoint: A,
    ) -> Result<P, Box<dyn Error>> {
        let mapjson = HashMap::new();
        post_api(self.get_current_server(), endpoint.as_ref(), mapjson).await
    }

    pub async fn get_server_config(&mut self) -> Result<ApiConfig, Box<dyn Error>> {
        Ok(self.post_api("/json/config").await?)
    }

    pub fn get_stations(&self) -> StationSearchBuilder {
        StationSearchBuilder::new(self.clone())
    }

    pub fn get_countries(&self) -> CountrySearchBuilder {
        CountrySearchBuilder::new(self.clone())
    }

    pub fn get_languages(&self) -> LanguageSearchBuilder {
        LanguageSearchBuilder::new(self.clone())
    }

    pub fn get_tags(&self) -> TagSearchBuilder {
        TagSearchBuilder::new(self.clone())
    }

    pub async fn send<P: AsRef<str>, Q: DeserializeOwned>(
        &mut self,
        endpoint: P,
        mapjson: HashMap<String, String>,
    ) -> Result<Q, Box<dyn Error>> {
        post_api(self.get_current_server(), endpoint, mapjson).await
    }

    pub async fn get_countries_filtered<P: AsRef<str>>(
        &mut self,
        filter: P,
    ) -> Result<Vec<ApiCountry>, Box<dyn Error>> {
        Ok(self
            .post_api(format!("/json/countries/{}", filter.as_ref()))
            .await?)
    }

    pub async fn get_servers() -> Result<Vec<String>, Box<dyn Error>> {
        let resolver = resolver(
            config::ResolverConfig::default(),
            config::ResolverOpts::default(),
        )
        .await?;
        let response = resolver
            .lookup(
                "_api._tcp.radio-browser.info",
                RecordType::SRV,
                DnsRequestOptions::default(),
            )
            .await?;
        let mut list: Vec<String> = response
            .iter()
            .filter_map(|entry| entry.as_srv())
            .map(|entry| entry.target().to_string().trim_matches('.').to_string())
            .collect();

        list.shuffle(&mut thread_rng());
        println!("Servers: {:?}", list);
        Ok(list)
    }
}
