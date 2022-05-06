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
    let mut api2 = api.clone();
    let config = api.get_server_config();
    let stats = api2.get_server_status();
    let (stations, config, countries, languages, tags, stats) = join!(stations, config, countries, languages, tags, stats);

    println!("Config: {:#?}", config?);
    println!("Status: {:#?}", stats?);
    println!("Countries found: {}", countries?.len());
    println!("Languages found: {}", languages?.len());
    let tags = tags?;
    println!("Tags found: {}", tags.len());
    println!("{:?}", tags);
    let stations = stations?;
    println!("Stations found: {}", stations.len());

    let vote_result = api.station_vote(&stations[0].stationuuid).await?;
    println!("Stations voted result: {:?}", vote_result);
    let click_result = api.station_click(&stations[0].stationuuid).await?;
    println!("Stations clicked result: {:?}", click_result);
    Ok(())
}
