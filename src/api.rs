// The Api file for extension.

// Copyright (C) 2022 SpringHan 

use ncmapi::NcmApi;
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};

static mut API: Option<NcmApi> = None;

pub trait SpecialJsonStructure {
    fn from_data<'a>(data: &'a Vec<u8>) -> Self
    where Self: Deserialize<'a>
    {
        serde_json::from_slice::<Self>(data).unwrap()
    }
}

/// The structure used for deserializing user info json
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    #[serde(default)]
    pub code: usize,

    #[serde(default)]
    pub account: Value,

    #[serde(default)]
    pub profile: Value,

    #[serde(default)]
    pub msg: Value
}

impl SpecialJsonStructure for UserInfo {}

/// The structure used for deserializing playlists json
#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistsInfo {
    #[serde(default)]
    pub more: Value,

    #[serde(default)]
    pub playlist: Value,

    #[serde(default)]
    pub code: usize
}

impl SpecialJsonStructure for PlaylistsInfo {}

/// The structure used for deserializing lyrics json
#[derive(Serialize, Deserialize, Debug)]
pub struct LyricsInfo {
    #[serde(default)]
    pub lrc: Value,

    #[serde(default)]
    pub tlyric: Value,

    #[serde(default)]
    pub code: usize
}

impl SpecialJsonStructure for LyricsInfo {}

/// The structure used for deserializing playlist json
#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistInfo {
    #[serde(default)]
    pub code: usize,

    #[serde(default)]
    pub id: i64,

    #[serde(default)]
    pub msg: Value
}

impl SpecialJsonStructure for PlaylistInfo {}

/// The structure used for deserializing recommended playlists
#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendPlaylists {
    #[serde(default)]
    pub code: usize,

    #[serde(default)]
    pub recommend: Value
}

impl SpecialJsonStructure for RecommendPlaylists {}

// Basic functions

/// Initialize the API variable
pub fn init_api() {
    unsafe {
        API = Some(NcmApi::default());
    }
}

/// Return API reference
fn get_api<'a>() -> &'a NcmApi {
    unsafe {
        match API {
            None => panic!("API hasn't been initialized!"),
            Some(ref api) => api
        }
    }
}

// Account functions
/// The function for login
pub async fn login(phone_number: &str, password: &str) -> Result<(Value, Value), Value> {
    let api = get_api();
    let result = UserInfo::from_data(
        api.login_phone(phone_number, password)
            .await
            .unwrap()
            .data()
    );
    if result.code == 200 {
        Ok((result.account, result.profile))
    } else {
        Err(result.msg)
    }
}

/// Login status
pub async fn login_status() -> Result<(Value, Value), Value> {
    let api = get_api();
    let status = UserInfo::from_data(
        api.login_status()
            .await
            .unwrap()
            .data()
    );
    match status.account {
        Value::Null => Err(Value::Null),
        _ => Ok((status.account, status.profile))
    }
}

/// Logout
pub async fn logout() -> Result<(), ()> {
    let api = get_api();
    let result = UserInfo::from_data(
        api.logout()
            .await
            .unwrap()
            .data()
    );
    if result.code == 200 {
        Ok(())
    } else {
        Err(())
    }
}

/// Create playlist
pub async fn create_playlist(name: String, privacy: bool) -> Result<i64, Value> {
    let api = get_api();
    let result = PlaylistInfo::from_data(
        api.create_playlist(name, privacy)
            .await
            .unwrap()
            .data()
    );
    if result.code == 200 {
        Ok(result.id)
    } else {
        Err(result.msg)
    }
}

/// Delete playlist
pub async fn delete_playlist(pid: i64) -> Result<(), ()> {
    let api = get_api();
    let result = api.delete_playlist(pid as usize)
        .await
        .unwrap()
        .deserialize_to_implict();
    if result.code == 200 {
        Ok(())
    } else {
        Err(())
    }
}

// NOTE: Maybe the type of `tracks` will be modified.
/// Track songs in playlist
pub async fn track(pid: i64, op: u8, tracks: Vec<usize>) -> Result<(), ()> {
    let api = get_api();
    let result = api.playlist_tracks(pid as usize, op, tracks)
        .await
        .unwrap()
        .deserialize_to_implict();
    if result.code == 200 {
        Ok(())
    } else {
        Err(())
    }
}

/// Rename playlist
pub async fn rename_playlist(pid: i64, name: String) -> Result<(), ()> {
    let api = get_api();
    let result = api.update_playlist_name(pid as usize, name)
        .await
        .unwrap()
        .deserialize_to_implict();
    if result.code == 200 {
        Ok(())
    } else {
        Err(())
    }
}

/// Change playlist order
pub async fn update_playlist_order(pid: i64, ids: Vec<usize>) -> Result<(), Value> {
    let api = get_api();
    let result = api.update_playlist_order(pid as usize, ids)
        .await
        .unwrap()
        .deserialize_to_implict();
    if result.code == 200 {
        Ok(())
    } else {
        Err(result.message)
    }
}

/// Get recommend songs
pub async fn recommend_songs() -> Result<Value, ()> {
    let api = get_api();
    let songs = api.recommend_songs()
        .await
        .unwrap()
        .deserialize_to_implict();
    if songs.code == 200 {
        Ok(songs.data.get("dailySongs").unwrap().to_owned())
    } else {
        Err(())
    }
}

/// Get recommend playlists
pub async fn recommend_playlists() -> Result<Value, ()> {
    let api = get_api();
    let playlists = RecommendPlaylists::from_data(
        api.recommend_resource()
            .await
            .unwrap()
            .data()
    );
    if playlists.code == 200 {
        Ok(playlists.recommend)
    } else {
        Err(())
    }
}

// Fundemantal functions
/// Search song.
pub async fn search(content: &str) -> Result<Value, Value> {
    let api = get_api();
    let result = api.cloud_search(content, Some(json!({ "limit": "1" })))
        .await
        .unwrap()
        .deserialize_to_implict();
    if result.code == 200 {
        if result.result.get("songCount").unwrap().to_string() == "0" {
            return Err(Value::Null)
        }

        Ok(result.result)
    } else {
        Err(result.msg)
    }
}

/// Get user's playlist.
pub async fn user_playlist(uid: i64) -> Result<(Value, Value), ()> {
    let api = get_api();
    let result = PlaylistsInfo::from_data(
        api.user_playlist(uid as usize, None)
            .await
            .unwrap()
            .data()
    );
    if result.playlist.as_array().unwrap().len() == 0 {
        Err(())
    } else {
        Ok((result.more, result.playlist))
    }
}

/// Get song's lyrics.
pub async fn get_lyrics(sid: i64) -> Result<(String, String), ()> {
    let api = get_api();
    let lyrics = LyricsInfo::from_data(
        api.lyric(sid as usize)
            .await
            .unwrap()
            .data());
    match lyrics.lrc {
        Value::Null => Err(()),
        _ => Ok((
            lyrics.lrc.get("lyric")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
            lyrics.tlyric.get("lyric")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned()
        ))
    }
}
