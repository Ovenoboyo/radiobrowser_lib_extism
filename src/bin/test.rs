use radiobrowser_lib_rust::RadioBrowserAPI;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut api = RadioBrowserAPI::new().await?;
    let config = api.get_server_config().await?;
    println!("{:#?}", config);
    let countries = api.get_countries().await?;
    println!("{:?}", countries);
    Ok(())
}
