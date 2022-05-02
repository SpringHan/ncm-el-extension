// Rust extension for netease-cloud-music.el

// Copyright (c) 2022 SpringHan

use ncmapi::NcmApi;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = NcmApi::default();
    let res = api.cloud_search("Loser", None).await;
    let r = res.unwrap().deserialize_to_implict();
    println!("{:#?}", r);
    Ok(())
}
