use crate::external::post_api;
use crate::ApiConfig;
use crate::ApiStationClickResult;
use crate::ApiStationHistory;
use crate::ApiStationVoteResult;
use crate::ApiStatus;
use crate::CountrySearchBuilder;
use crate::LanguageSearchBuilder;
use crate::RbError;
use crate::StationSearchBuilder;
use crate::TagSearchBuilder;

use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use rand::seq::SliceRandom;
#[cfg(not(target_arch = "wasm32"))]
use rand::thread_rng;

use log::trace;

#[cfg(not(target_arch = "wasm32"))]
use async_std_resolver::proto::rr::RecordType;
#[cfg(not(target_arch = "wasm32"))]
use async_std_resolver::{config, resolver};

/// RadioBrowser client for async communication
///
/// It uses crate:async_std
///
/// Example
/// ```rust
/// use radiobrowser::RbError;
/// use radiobrowser::RadioBrowserAPI;
/// #[async_std::main]
/// async fn main() -> Result<(), RbError> {
///     let mut api = RadioBrowserAPI::new().await?;
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug)]
pub struct RadioBrowserAPI {
    servers: Vec<String>,
    current: usize,
}

impl RadioBrowserAPI {
    /// Create a new instance of a radiobrowser api client.
    /// It will fetch a list of radiobrowser server with get_default_servers()
    /// and save it internally.
    pub async fn new() -> Result<Self, RbError> {
        Ok(RadioBrowserAPI {
            servers: RadioBrowserAPI::get_default_servers().await?,
            current: 0,
        })
    }

    /// Create a new instance of a radiobrowser api client from
    /// a single dns name. Use this is you want to connect to a single named server.
    pub async fn new_from_dns_a<P: AsRef<str>>(dnsname: P) -> Result<Self, RbError> {
        Ok(RadioBrowserAPI {
            servers: vec![dnsname.as_ref().to_string()],
            current: 0,
        })
    }

    /// Create a new instance of a radiobrowser api client from
    /// a dns srv record which may have multiple dns A/AAAA records.
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn new_from_dns_srv<P: AsRef<str>>(srvname: P) -> Result<Self, RbError> {
        Ok(RadioBrowserAPI {
            servers: RadioBrowserAPI::get_servers_from_dns_srv(srvname).await?,
            current: 0,
        })
    }

    fn get_current_server(&mut self) -> String {
        trace!("get_current_server()");
        if self.servers.len() > 0 {
            self.current = self.current % self.servers.len();
            format!("https://{}", self.servers[self.current])
        } else {
            String::from("https://de1.api.radio-browser.info")
        }
    }

    async fn post_api<P: DeserializeOwned, A: AsRef<str>>(
        &mut self,
        endpoint: A,
    ) -> Result<P, RbError> {
        let mapjson = HashMap::new();
        post_api(self.get_current_server(), endpoint.as_ref(), mapjson).await
    }

    pub async fn get_station_changes(
        &mut self,
        limit: u64,
        last_change_uuid: Option<String>,
    ) -> Result<Vec<ApiStationHistory>, RbError> {
        let query = match last_change_uuid {
            Some(uuid) => format!(
                "/json/stations/changed?limit={}&lastchangeuuid={}",
                limit, uuid
            ),
            None => format!("/json/stations/changed?limit={}", limit),
        };
        Ok(self.post_api(query).await?)
    }

    pub async fn get_server_config(&mut self) -> Result<ApiConfig, RbError> {
        Ok(self.post_api("/json/config").await?)
    }

    pub async fn get_server_status(&mut self) -> Result<ApiStatus, RbError> {
        Ok(self.post_api("/json/stats").await?)
    }

    /// Add a click to a station found by stationuuid
    pub async fn station_click<P: AsRef<str>>(
        &mut self,
        stationuuid: P,
    ) -> Result<ApiStationClickResult, RbError> {
        Ok(self
            .post_api(format!("/json/url/{}", stationuuid.as_ref()))
            .await?)
    }

    /// Add a vote to a station found by a stationuuid
    pub async fn station_vote<P: AsRef<str>>(
        &mut self,
        stationuuid: P,
    ) -> Result<ApiStationVoteResult, RbError> {
        Ok(self
            .post_api(format!("/json/vote/{}", stationuuid.as_ref()))
            .await?)
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
    ) -> Result<Q, RbError> {
        post_api(self.get_current_server(), endpoint, mapjson).await
    }

    pub async fn get_default_servers() -> Result<Vec<String>, RbError> {
        trace!("get_default_servers()");
        #[cfg(not(target_arch = "wasm32"))]
        {
            RadioBrowserAPI::get_servers_from_dns_srv("_api._tcp.radio-browser.info").await
        }

        #[cfg(target_arch = "wasm32")]
        {
            RadioBrowserAPI::get_servers_from_http().await
        }
    }

    pub async fn get_servers_from_http() -> Result<Vec<String>, RbError> {
        let response = reqwest::get("http://all.api.radio-browser.info/json/servers").await?;
        let servers: Vec<HashMap<String, String>> = response.json().await?;
        Ok(servers
            .into_iter()
            .filter_map(|server| server.get("name").cloned())
            .collect())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn get_servers_from_dns_srv<P: AsRef<str>>(srvname: P) -> Result<Vec<String>, RbError> {
        trace!("get_servers_from_dns_srv()");
        let resolver = resolver(
            config::ResolverConfig::default(),
            config::ResolverOpts::default(),
        )
        .await;
        let response = resolver.lookup(srvname.as_ref(), RecordType::SRV).await?;
        let mut list: Vec<String> = response
            .iter()
            .filter_map(|entry| entry.as_srv())
            .map(|entry| entry.target().to_string().trim_matches('.').to_string())
            .collect();

        list.shuffle(&mut thread_rng());
        trace!("Servers: {:?}", list);
        Ok(list)
    }
}
