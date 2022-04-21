use futures::join;
use radiobrowser_lib_rust::RadioBrowserAPI;
use radiobrowser_lib_rust::StationOrder;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut api = RadioBrowserAPI::new().await?;
    let countries = api.get_countries().filter("a").send();
    let stations = api
        .search()
        .name("jazz")
        .reverse(true)
        .order(StationOrder::Clickcount)
        .send();
    let config = api.get_server_config();
    let (stations, config, countries) = join!(stations, config, countries);

    println!("Config: {:#?}", config?);
    println!("Countries found: {}", countries?.len());
    println!("Stations found: {}", stations?.len());
    Ok(())
}
