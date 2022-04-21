# Radiobrowser Lib Rust
Client library for radio-browser.info and other radio-browser-rust servers

## Getting started
Cargo.toml entry
```toml
radiobrowser = "*"
```
Example:
```rust
use radiobrowser::RadioBrowserAPI;
use radiobrowser::StationOrder;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut api = RadioBrowserAPI::new().await?;
    let stations = api
        .get_stations()
        .name("jazz")
        .reverse(true)
        .order(StationOrder::Clickcount)
        .send()
        .await?;

    println!("Stations found: {}", stations?.len());
    Ok(())
}
```

## Usage
Documentation is at https://docs.rs/radiobrowser

## License
This project is MIT licensed.
