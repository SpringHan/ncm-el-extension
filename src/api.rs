// The Api file for extension.

// Copyright (C) 2022 SpringHan

use ncmapi::NcmApi;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

static mut API: Option<NcmApi> = None;

pub trait SpecialJsonStructure {
    fn from_data<'a>(data: &'a Vec<u8>) -> Self
    where
        Self: Deserialize<'a>,
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
    pub msg: Value,
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
    pub code: usize,
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
    pub code: usize,
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
    pub msg: Value,
}

impl SpecialJsonStructure for PlaylistInfo {}

/// The structure used for deserializing recommended playlists
#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendPlaylists {
    #[serde(default)]
    pub code: usize,

    #[serde(default)]
    pub recommend: Value,
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
            Some(ref api) => api,
        }
    }
}

// Account functions
/// The function for login
pub async fn login(
    phone_number: String,
    password: String,
) -> Result<(i64, String, String, String, String), Value> {
    let api = get_api();
    let result = UserInfo::from_data(
        api.login_phone(&phone_number, &password)
            .await
            .unwrap()
            .data(),
    );
    if result.code == 200 {
        let profile = result.profile;
        Ok((
            result.account.get("id").unwrap().as_i64().unwrap(),
            profile
                .get("nickname")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
            phone_number,
            password,
            profile
                .get("avatarUrl")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
        ))
    } else {
        Err(result.msg)
    }
}

/// Check if logged
pub async fn login_status() -> Result<(), ()> {
    let api = get_api();
    let status = UserInfo::from_data(api.login_status().await.unwrap().data());
    match status.account {
        Value::Null => Err(()),
        _ => Ok(()),
    }
}

/// Logout
pub async fn logout() -> Result<(), ()> {
    let api = get_api();
    let result = UserInfo::from_data(api.logout().await.unwrap().data());
    if result.code == 200 {
        Ok(())
    } else {
        Err(())
    }
}

/// Create playlist
pub async fn create_playlist(name: String, privacy: bool) -> Result<i64, Value> {
    let api = get_api();
    let result = PlaylistInfo::from_data(api.create_playlist(name, privacy).await.unwrap().data());
    if result.code == 200 {
        Ok(result.id)
    } else {
        Err(result.msg)
    }
}

/// Delete playlist
pub async fn delete_playlist(pid: i64) -> Result<(), ()> {
    let api = get_api();
    let result = api
        .delete_playlist(pid as usize)
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
pub async fn track(pid: i64, op: i64, tracks: Vec<usize>) -> Result<(), ()> {
    let api = get_api();
    let result = api
        .playlist_tracks(pid as usize, op as u8, tracks)
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
    let result = api
        .update_playlist_name(pid as usize, name)
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
    let result = api
        .update_playlist_order(pid as usize, ids)
        .await
        .unwrap()
        .deserialize_to_implict();
    if result.code == 200 {
        Ok(())
    } else {
        Err(result.message)
    }
}

/// Extract songs' main info from json data
fn extract_songs_info(json_data: &Value) -> Result<Vec<(i64, String, String)>, ()> {
    let mut result = Vec::<(i64, String, String)>::new();
    for i in json_data.as_array().unwrap().iter() {
        let artist = i.get("ar").unwrap().as_array().unwrap().first().unwrap();
        result.push((
            i.get("id").unwrap().as_i64().unwrap(),
            i.get("name").unwrap().as_str().unwrap().to_owned(),
            artist.get("name").unwrap().as_str().unwrap().to_owned(),
        ));
    }

    Ok(result)
}

/// Get recommend songs
pub async fn recommend_songs() -> Result<Vec<(i64, String, String)>, ()> {
    let api = get_api();
    let songs = api
        .recommend_songs()
        .await
        .unwrap()
        .deserialize_to_implict();
    if songs.code == 200 {
        let songs = songs.data.get("dailySongs").unwrap();
        Ok(extract_songs_info(songs).unwrap())
    } else {
        Err(())
    }
}

/// Extract playlists' info from json data
fn extract_playlists_info(json_data: &Value) -> Result<Vec<(i64, String)>, ()> {
    let mut result = Vec::<(i64, String)>::new();
    for i in json_data.as_array().unwrap().iter() {
        result.push((
            i.get("id").unwrap().as_i64().unwrap(),
            i.get("name").unwrap().as_str().unwrap().to_owned(),
        ))
    }

    Ok(result)
}

/// Get recommend playlists
pub async fn recommend_playlists() -> Result<Vec<(i64, String)>, ()> {
    let api = get_api();
    let playlists = RecommendPlaylists::from_data(api.recommend_resource().await.unwrap().data());
    if playlists.code == 200 {
        Ok(extract_playlists_info(&playlists.recommend).unwrap())
    } else {
        Err(())
    }
}

// Fundemantal functions
/// Search song
pub async fn search_song(
    content: &str,
    limit: i64,
    page: i64,
) -> Result<Vec<(i64, String, String)>, Value> {
    let api = get_api();
    let result = api
        .cloud_search(
            content,
            Some(json!({ "limit": limit, "offset": page - 1  })),
        )
        .await
        .unwrap()
        .deserialize_to_implict();

    if result.code == 200 {
        if result.result.get("songCount").unwrap().to_string() == "0" {
            return Err(Value::Null);
        }

        Ok(extract_songs_info(result.result.get("songs").unwrap()).unwrap())
    } else {
        Err(result.msg)
    }
}

pub async fn search_playlist(
    content: &str,
    limit: i64,
    page: i64,
) -> Result<Vec<(i64, String)>, ()> {
    let api = get_api();
    let playlists = api
        .cloud_search(
            content,
            Some(json!({ "limit": limit,
                     "offset": page - 1,
                     "type": 1000i16
            })),
        )
        .await
        .unwrap()
        .deserialize_to_implict();

    if playlists.code == 200 {
        let playlists = playlists.result.get("playlists").unwrap();

        if playlists.as_array().unwrap().len() == 0 {
            Err(())
        } else {
            Ok(extract_playlists_info(&playlists).unwrap())
        }
    } else {
        Err(())
    }
}

/// Get user's playlist.
pub async fn user_playlist(uid: i64) -> Result<Vec<(i64, String)>, ()> {
    let api = get_api();
    let result =
        PlaylistsInfo::from_data(api.user_playlist(uid as usize, None).await.unwrap().data());
    let playlists = result.playlist.as_array().unwrap();

    if playlists.len() == 0 {
        Err(())
    } else {
        // NOTE: Maybe now I'll not use `more` to know whether there're other results.
        Ok(
            extract_playlists_info(&result.playlist).unwrap(), // result.more
        )
    }
}

/// Get song's lyrics.
pub async fn get_lyrics(sid: i64) -> Result<(String, String), ()> {
    let api = get_api();
    let lyrics = LyricsInfo::from_data(api.lyric(sid as usize).await.unwrap().data());
    match lyrics.lrc {
        Value::Null => Err(()),
        _ => Ok((
            lyrics
                .lrc
                .get("lyric")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
            lyrics
                .tlyric
                .get("lyric")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
        )),
    }
}

// #[tokio::main]
// async fn main() {
//     init_api();
//     // let a = search_playlist("Lemon", 1, 1).await.unwrap();
//     // println!("{:#?}", a);
//     // let b = search_song("Lemon", 1, 1).await.unwrap();
//     // println!("{:#?}", b);
//     // let a = get_comment(536622304, 1).await.unwrap();
//     // println!("{:#?}", a);
//     // let a = login(.to_string(), .to_string()).await.unwrap_or_default();
//     // println!("{:#?}", a);
//     // println!("{:#?}", login_status().await.unwrap());
//     // let a = recommend_playlists().await.unwrap();
//     // println!("{:#?}", a);
//     // let b = user_playlist().await.unwrap();
//     // println!("{:#?}", b);
//     // logout().await;
// }

/// Get Comment of current song
pub async fn get_comment(
    sid: i64,
    page_no: i64,
) -> Result<(Vec<(i64, String, String, String)>, bool), ()> {
    let api = get_api();
    let result = api
        .comment(
            sid as usize,
            ncmapi::ResourceType::Song,
            20,
            page_no as usize,
            1,
            0,
            true,
        )
        .await
        .unwrap()
        .deserialize_to_implict();
    let (has_more, result) = (
        result.data.get("hasMore").unwrap().as_bool().unwrap(),
        result.data.get("comments").unwrap(),
    );
    let mut results = Vec::<(i64, String, String, String)>::new();

    for i in result.as_array().unwrap().iter() {
        let user = i.get("user").unwrap();
        results.push((
            i.get("commentId").unwrap().as_i64().unwrap().to_owned(),
            i.get("content").unwrap().as_str().unwrap().to_owned(),
            user.get("nickname").unwrap().as_str().unwrap().to_owned(),
            user.get("avatarUrl").unwrap().as_str().unwrap().to_owned(),
        ));
    }
    Ok((results, has_more))
}

/// Create a new comment
pub async fn create_comment(sid: i64, content: &str, cid: i64) -> Result<(), ()> {
    let api = get_api();
    let result = if cid > 0 {
        api.comment_create(sid as usize, ncmapi::ResourceType::Song, content)
            .await
            .unwrap()
            .deserialize_to_implict()
    } else {
        api.comment_re(
            sid as usize,
            ncmapi::ResourceType::Song,
            cid as usize,
            content,
        )
        .await
        .unwrap()
        .deserialize_to_implict()
    };

    if result.code == 200 {
        Ok(())
    } else {
        Err(())
    }
}
