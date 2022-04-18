use radiobrowser_lib_rust::ApiConfig;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let servers = radiobrowser_lib_rust::get_servers().await?;
    println!("Servers: {:?}", servers);
    let config: ApiConfig = radiobrowser_lib_rust::get_server_config().await?;
    println!("{:#?}", config);
    Ok(())
}
