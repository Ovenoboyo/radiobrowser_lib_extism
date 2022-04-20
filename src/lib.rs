use reqwest;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

use rand::seq::SliceRandom;
use rand::thread_rng;

use async_std_resolver::proto::rr::RecordType;
use async_std_resolver::proto::xfer::DnsRequestOptions;
use async_std_resolver::{config, resolver};

use chrono::DateTime;
use chrono::Utc;

#[derive(PartialEq, Deserialize, Debug)]
pub struct ApiStation {
    pub changeuuid: String,
    pub stationuuid: String,
    pub serveruuid: Option<String>,
    pub name: String,
    pub url: String,
    pub url_resolved: String,
    pub homepage: String,
    pub favicon: String,
    pub tags: String,
    pub country: String,
    pub countrycode: String,
    pub iso_3166_2: Option<String>,
    pub state: String,
    pub language: String,
    pub languagecodes: Option<String>,
    pub votes: i32,
    pub lastchangetime: String,
    pub lastchangetime_iso8601: Option<DateTime<Utc>>,
    pub codec: String,
    pub bitrate: u32,
    pub hls: i8,
    pub lastcheckok: i8,
    pub lastchecktime: String,
    pub lastchecktime_iso8601: Option<DateTime<Utc>>,
    pub lastcheckoktime: String,
    pub lastcheckoktime_iso8601: Option<DateTime<Utc>>,
    pub lastlocalchecktime: String,
    pub lastlocalchecktime_iso8601: Option<DateTime<Utc>>,
    pub clicktimestamp: String,
    pub clicktimestamp_iso8601: Option<DateTime<Utc>>,
    pub clickcount: u32,
    pub clicktrend: i32,
    pub ssl_error: Option<u8>,
    pub geo_lat: Option<f64>,
    pub geo_long: Option<f64>,
    pub has_extended_info: Option<bool>,
}

#[derive(PartialEq, Eq, Deserialize, Debug)]
pub struct ApiCountry {
    pub name: String,
    pub iso_3166_1: String,
    pub stationcount: u32,
}

#[derive(PartialEq, Eq, Deserialize, Debug)]
pub struct ApiLanguage {
    pub name: String,
    pub iso_639: Option<String>,
    pub stationcount: u32,
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct ApiStreamingServer {
    pub uuid: String,
    pub url: String,
    pub statusurl: Option<String>,
    pub status: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub check_enabled: bool,
    pub prometheus_exporter_enabled: bool,
    pub pull_servers: Vec<String>,
    pub tcp_timeout_seconds: u32,
    pub broken_stations_never_working_timeout_seconds: u32,
    pub broken_stations_timeout_seconds: u32,
    pub checks_timeout_seconds: u32,
    pub click_valid_timeout_seconds: u32,
    pub clicks_timeout_seconds: u32,
    pub mirror_pull_interval_seconds: u32,
    pub update_caches_interval_seconds: u32,
    pub server_name: String,
    pub server_location: String,
    pub server_country_code: String,
    pub check_retries: u32,
    pub check_batchsize: u32,
    pub check_pause_seconds: u32,
    pub api_threads: u32,
    pub cache_type: String,
    pub cache_ttl: u32,
    pub language_replace_filepath: String,
    pub language_to_code_filepath: String,
}

#[derive(Clone, Debug)]
pub struct SearchBuilder {
    map: HashMap<String, String>,
    api: RadioBrowserAPI,
}

impl SearchBuilder {
    pub fn new(api: RadioBrowserAPI) -> Self {
        SearchBuilder {
            api,
            map: HashMap::new(),
        }
    }

    pub fn name<P: AsRef<str>>(mut self, name: P) -> Self {
        self.map
            .insert(String::from("name"), name.as_ref().to_string());
        self
    }

    pub fn countrycode<P: AsRef<str>>(mut self, countrycode: P) -> Self {
        self.map.insert(
            String::from("countrycode"),
            countrycode.as_ref().to_string(),
        );
        self
    }

    pub async fn send(mut self) -> Result<Vec<ApiStation>, Box<dyn Error>> {
        Ok(self.api.send("/json/stations/search", self.map).await?)
    }
}

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

    pub fn search(&self) -> SearchBuilder {
        SearchBuilder::new(self.clone())
    }

    pub async fn get_server_config(&mut self) -> Result<ApiConfig, Box<dyn Error>> {
        Ok(self.post_api("/json/config").await?)
    }

    pub async fn get_countries(&mut self) -> Result<Vec<ApiCountry>, Box<dyn Error>> {
        Ok(self.post_api("/json/countries").await?)
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

pub async fn post_api<P: DeserializeOwned, A: AsRef<str>, B: AsRef<str>>(
    server: A,
    endpoint: B,
    mapjson: HashMap<String, String>,
) -> Result<P, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}{}", server.as_ref(), endpoint.as_ref()))
        .json(&mapjson)
        .send()
        .await?
        .json::<P>()
        .await?;
    Ok(res)
}
