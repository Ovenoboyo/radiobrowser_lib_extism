//! 
//!
//! # Example
//! ```rust
//! use std::error::Error;
//! use futures::join;
//! use radiobrowser::RadioBrowserAPI;
//! use radiobrowser::StationOrder;
//! 
//! #[async_std::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let mut api = RadioBrowserAPI::new().await?;
//!     let countries = api.get_countries().send();
//!     let languages = api.get_languages().send();
//!     let stations = api
//!         .get_stations()
//!         .name("jazz")
//!         .reverse(true)
//!         .order(StationOrder::Clickcount)
//!         .send();
//!     let config = api.get_server_config();
//!     let (stations, config, countries, languages) = join!(stations, config, countries, languages);
//! 
//!     println!("Config: {:#?}", config?);
//!     println!("Countries found: {}", countries?.len());
//!     println!("Languages found: {}", languages?.len());
//!     println!("Stations found: {}", stations?.len());
//!     Ok(())
//! }
//! ```

mod api;
mod external;
mod stationsearchbuilder;
mod countrysearchbuilder;
mod languagesearchbuilder;
mod tagsearchbuilder;
mod structs;

pub use api::RadioBrowserAPI;
pub use structs::ApiConfig;
pub use structs::ApiCountry;
pub use structs::ApiLanguage;
pub use structs::ApiStation;
pub use structs::ApiStreamingServer;
pub use structs::ApiTag;
pub use stationsearchbuilder::StationSearchBuilder;
pub use stationsearchbuilder::StationOrder;
pub use countrysearchbuilder::CountrySearchBuilder;
pub use languagesearchbuilder::LanguageSearchBuilder;
pub use tagsearchbuilder::TagSearchBuilder;