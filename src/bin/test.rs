/*
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
*/

use radiobrowser::blocking::RadioBrowserAPI;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let api = RadioBrowserAPI::new()?;
    let servers = RadioBrowserAPI::get_servers()?;
    println!("Servers: {:?}", servers);
    let countries = api.get_countries().send()?;
    println!("Countries: {:?}", countries);
    let stations = api.get_stations().name("jazz").send()?;
    println!("Stations: {:?}", stations);
    Ok(())
}