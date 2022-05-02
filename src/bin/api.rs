// The Api file for extension.

// Copyright (C) 2022 SpringHan 

use ncmapi::NcmApi;
use serde_json::{json, Value};
use std::future;

static mut API: Option<NcmApi> = None;

#[tokio::main]
async fn main() {
    init_api();
    logout().await;
    // login_status().await;
}

/// Initialize the API variable.
fn init_api() {
    unsafe {
        API = Some(NcmApi::default());
    }
}

/// Return API reference.
fn get_api<'a>() -> &'a NcmApi {
    unsafe {
        match API {
            None => panic!("API hasn't been initialized!"),
            Some(ref api) => api
        }
    }
}

/// The function for login.
async fn login(phone_number: &str, password: &str) -> Result<(), ()> {
    let api = get_api();
    let result = api.login_phone(phone_number, password)
        .await
        .unwrap()
        .deserialize_to_implict();
    println!("{:#?}", result);
    Ok(())
}

/// Login status.
async fn login_status() -> Result<(), ()> {
    let api = get_api();
    let result = api.login_status()
        .await
        .unwrap()
        .deserialize_to_implict();
    println!("{:#?}", result);
    Ok(())
}

async fn logout() -> Result<(), ()> {
    let api = get_api();
    let result = api.logout()
        .await
        .unwrap()
        .deserialize_to_implict();
    println!("{:#?}", result);
    Ok(())
}
