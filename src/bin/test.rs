use radiobrowser::blocking::RadioBrowserAPI;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut api = RadioBrowserAPI::new()?;
    let servers = RadioBrowserAPI::get_default_servers()?;
    println!("Servers: {:?}", servers);
    let status = api.get_server_status()?;
    println!("Status: {:#?}", status);
    let config = api.get_server_config()?;
    println!("Config: {:#?}", config);
    let countries = api.get_countries().send()?;
    println!("Countries: {:?}", countries.len());
    let tags = vec!["jazz","classical"];
    let stations = api.get_stations().tag_list(tags).send()?;
    println!("Stations with tags containing 'jazz' and 'classical': {:?}", stations.len());
    println!("First found station: {:#?}", stations[0].clicktimestamp_iso8601);
    let vote_result = api.station_vote(&stations[0].stationuuid)?;
    println!("Stations voted result: {:?}", vote_result);
    let click_result = api.station_click(&stations[0].stationuuid)?;
    println!("Stations clicked result: {:?}", click_result);
    let station_changes = api.get_station_changes(1,None)?;
    println!("Station changes result: {:#?}", station_changes);
    Ok(())
}
