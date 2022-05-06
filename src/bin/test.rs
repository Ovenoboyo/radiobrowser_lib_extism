use radiobrowser::blocking::RadioBrowserAPI;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut api = RadioBrowserAPI::new()?;
    let servers = RadioBrowserAPI::get_servers()?;
    println!("Servers: {:?}", servers);
    let status = api.get_server_status()?;
    println!("Status: {:#?}", status);
    let config = api.get_server_config()?;
    println!("Config: {:#?}", config);
    let countries = api.get_countries().send()?;
    println!("Countries: {:?}", countries.len());
    let stations = api.get_stations().name("jazz").send()?;
    println!("Stations with name containing 'jazz': {:?}", stations.len());
    let vote_result = api.station_vote(&stations[0].stationuuid)?;
    println!("Stations voted result: {:?}", vote_result);
    let click_result = api.station_click(&stations[0].stationuuid)?;
    println!("Stations clicked result: {:?}", click_result);
    Ok(())
}
