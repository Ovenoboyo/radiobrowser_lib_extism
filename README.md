# Radiobrowser Lib Rust
Client library for radio-browser.info and other radio-browser-rust servers

## Getting started
Cargo.toml entry
```toml
radiobrowser = "*"
```
Example:
```rust
use futures::join;
use radiobrowser::RadioBrowserAPI;
use radiobrowser::StationOrder;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut api = RadioBrowserAPI::new().await?;
    let countries = api.get_countries().send();
    let languages = api.get_languages().send();
    let tags = api.get_tags().filter("jazz").send();
    let stations = api
        .get_stations()
        .name("jazz")
        .reverse(true)
        .order(StationOrder::Clickcount)
        .send();
    let config = api.get_server_config();
    let (stations, config, countries, languages, tags) = join!(stations, config, countries, languages, tags);

    println!("Config: {:#?}", config?);
    println!("Countries found: {}", countries?.len());
    println!("Languages found: {}", languages?.len());
    let tags = tags?;
    println!("Tags found: {}", tags.len());
    println!("{:?}", tags);
    println!("Stations found: {}", stations?.len());
    Ok(())
}
```

## Installation
Within a particular ecosystem, there may be a common way of installing things, such as using Yarn, NuGet, or Homebrew. However, consider the possibility that whoever is reading your README is a novice and would like more guidance. Listing specific steps helps remove ambiguity and gets people to using your project as quickly as possible. If it only runs in a specific context like a particular programming language version or operating system or has dependencies that have to be installed manually, also add a Requirements subsection.

## Usage
Documentation is at https://docs.rs/radiobrowser

## License
This project is MIT licensed.
