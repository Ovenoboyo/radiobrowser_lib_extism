use reqwest;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::error::Error;

pub async fn post_api<P: DeserializeOwned, A: AsRef<str>, B: AsRef<str>>(
    server: A,
    endpoint: B,
    mapjson: HashMap<String, String>,
) -> Result<P, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}{}", server.as_ref(), endpoint.as_ref()))
        .json(&mapjson)
        .send()
        .await?
        .json::<P>()
        .await?;
    Ok(res)
}
