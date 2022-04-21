use serde::Deserialize;
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

#[derive(PartialEq, Eq, Deserialize, Debug)]
pub struct ApiTag {
    pub name: String,
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