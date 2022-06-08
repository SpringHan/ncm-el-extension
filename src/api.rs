// The Api file for extension.

// Copyright (C) 2022 SpringHan

use emacs::Result as EResult;
use emacs::Value as EValue;
use emacs::{defun, Env, FromLisp, IntoLisp};
use ncmapi::NcmApi;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JValue;

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
    pub account: JValue,

    #[serde(default)]
    pub profile: JValue,

    #[serde(default)]
    pub msg: JValue,
}

impl SpecialJsonStructure for UserInfo {}

/// The structure used for deserializing playlists json
#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistsInfo {
    #[serde(default)]
    pub more: JValue,

    #[serde(default)]
    pub playlist: JValue,

    #[serde(default)]
    pub code: usize,
}

impl SpecialJsonStructure for PlaylistsInfo {}

/// The structure used for deserializing lyrics json
#[derive(Serialize, Deserialize, Debug)]
pub struct LyricsInfo {
    #[serde(default)]
    pub lrc: JValue,

    #[serde(default)]
    pub tlyric: JValue,

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
    pub msg: JValue,
}

impl SpecialJsonStructure for PlaylistInfo {}

/// The structure used for deserializing recommended playlists
#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendPlaylists {
    #[serde(default)]
    pub code: usize,

    #[serde(default)]
    pub recommend: JValue,
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
/// Login with your PHONE number and PASSWORD.
#[defun]
#[tokio::main]
pub async fn login(env: &Env, phone: i64, password: String) -> EResult<EValue<'_>> {
    let api = get_api();
    let result = UserInfo::from_data(
        api.login_phone(&phone.to_string(), &password)
            .await
            .unwrap()
            .data(),
    );
    if result.code == 200 {
        let profile = result.profile;
        Ok(env.list((
            result.account.get("id").unwrap().as_i64().unwrap(),
            profile
                .get("nickname")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
            profile
                .get("avatarUrl")
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
        ))?)
    } else {
        Ok(().into_lisp(env)?)
    }
}

/// Check if you've loginned. If that's true, return t. Otherwise return nil.
#[defun]
#[tokio::main]
pub async fn loginp() -> EResult<bool> {
    let api = get_api();
    let status = UserInfo::from_data(api.login_status().await.unwrap().data());
    match status.account {
        JValue::Null => Ok(false),
        _ => Ok(true),
    }
}

/// Logout. If doing it successfully, it'll return t.
/// If failing, it'll return 0.
/// If you haven't loginned, return nil.
#[defun]
#[tokio::main]
pub async fn logout(env: &Env) -> EResult<EValue<'_>> {
    if loginp().unwrap() {
        let api = get_api();
        let result = UserInfo::from_data(api.logout().await.unwrap().data());
        if result.code == 200 {
            Ok(true.into_lisp(env)?)
        } else {
            Ok(0i64.into_lisp(env)?)
        }
    } else {
        Ok(().into_lisp(env)?)
    }
}

/// Create a new playlist named NAME.
/// If privacy is non-nil, then the playlist will be privacy.
/// Otherwise it'll be public.
#[defun]
#[tokio::main]
pub async fn create_playlist(name: String, privacy: EValue<'_>) -> EResult<Option<i64>> {
    let api = get_api();
    let result = PlaylistInfo::from_data(
        api.create_playlist(name, privacy.is_not_nil())
            .await
            .unwrap()
            .data(),
    );
    if result.code == 200 {
        Ok(Some(result.id))
    } else {
        Ok(None)
    }
}

/// Delete the user's playlist with PID.
#[defun]
#[tokio::main]
pub async fn delete_playlist(pid: i64) -> EResult<bool> {
    let api = get_api();
    let result = api
        .delete_playlist(pid as usize)
        .await
        .unwrap()
        .deserialize_to_implict();
    if result.code == 200 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Convert Lisp list into Vec.
/// The functions only can be used for list which has same atoms.
fn list_to_vec<'a, T>(list: EValue<'a>) -> EResult<Vec<T>>
where
    T: FromLisp<'a>,
{
    let mut result: Vec<T> = Vec::new();
    for i in 0..list.env.call("length", [list])?.into_rust::<i64>()? {
        let element = list.env.call("nth", (i, list))?.into_rust::<T>()?;
        result.push(element);
    }
    Ok(result)
}

// NOTE: Maybe the type of `tracks` will be modified.
/// Add or delete TRACKS with playlist whose id is PID.
/// If ADD is non-nil, add songs. Otherwise delete songs.
#[defun]
#[tokio::main]
pub async fn track(add: EValue<'_>, pid: i64, tracks: EValue<'_>) -> EResult<bool> {
    let op = if add.is_not_nil() { 1 } else { 0 };
    let api = get_api();
    let result = api
        .playlist_tracks(pid as usize, op as u8, list_to_vec(tracks)?)
        .await
        .unwrap()
        .deserialize_to_implict();
    if result.code == 200 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Rename playlist
#[defun]
#[tokio::main]
pub async fn rename_playlist(pid: i64, name: String) -> EResult<bool> {
    let api = get_api();
    let result = api
        .update_playlist_name(pid as usize, name)
        .await
        .unwrap()
        .deserialize_to_implict();
    if result.code == 200 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Update the songs' order in the playlist.
/// PID is the id of the playlist, SIDS is the list of songs' ids.
#[defun]
#[tokio::main]
pub async fn update_playlist_order(pid: i64, sids: EValue<'_>) -> EResult<bool> {
    let api = get_api();
    let result = api
        .update_playlist_order(pid as usize, list_to_vec(sids)?)
        .await
        .unwrap()
        .deserialize_to_implict();
    if result.code == 200 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Extract songs' main info from json data
fn extract_songs_info(env: &Env, json_data: JValue) -> Result<EValue<'_>, ()> {
    let mut result = Vec::<EValue<'_>>::new();
    for i in json_data.as_array().unwrap().iter() {
        let artist = i.get("ar").unwrap().as_array().unwrap().first().unwrap();
        result.push(
            env.list((
                i.get("id").unwrap().as_i64().unwrap(),
                i.get("name").unwrap().as_str().unwrap().to_owned(),
                artist.get("name").unwrap().as_str().unwrap().to_owned(),
            ))
            .unwrap(),
        );
    }

    Ok(env.list(&result.to_vec()).unwrap())
}

/// Get recommend songs
#[defun]
#[tokio::main]
pub async fn recommend_songs(env: &Env) -> EResult<EValue<'_>> {
    let api = get_api();
    let songs = api
        .recommend_songs()
        .await
        .unwrap()
        .deserialize_to_implict();
    if songs.code == 200 {
        let songs = songs.data.get("dailySongs").unwrap();
        Ok(extract_songs_info(env, songs.to_owned()).unwrap())
    } else {
        Ok(().into_lisp(env)?)
    }
}

/// Extract playlists' info from json data
fn extract_playlists_info(env: &Env, json_data: JValue) -> Result<EValue<'_>, ()> {
    let mut result = Vec::<EValue<'_>>::new();
    for i in json_data.as_array().unwrap().iter() {
        result.push(
            env.list((
                i.get("id").unwrap().as_i64().unwrap(),
                i.get("name").unwrap().as_str().unwrap().to_owned(),
            ))
            .unwrap(),
        )
    }

    Ok(env.list(&result).unwrap())
}

/// Get recommend playlists
#[defun]
#[tokio::main]
pub async fn recommend_playlists(env: &Env) -> EResult<EValue<'_>> {
    let api = get_api();
    let playlists = RecommendPlaylists::from_data(api.recommend_resource().await.unwrap().data());
    if playlists.code == 200 {
        Ok(extract_playlists_info(env, playlists.recommend).unwrap())
    } else {
        Ok(().into_lisp(env)?)
    }
}

// Fundemantal functions
/// Search song
pub async fn search_song<'a>(
    env: &'a Env,
    content: String,
    limit: i64,
    page: i64,
) -> EResult<EValue<'a>> {
    let api = get_api();
    let result = api
        .cloud_search(
            &content,
            Some(json!({ "limit": limit, "offset": page - 1  })),
        )
        .await
        .unwrap()
        .deserialize_to_implict();

    if result.code == 200 {
        let result = result.result;
        if result.get("songCount").unwrap().to_string() == "0" {
            return Ok(().into_lisp(env)?);
        }

        Ok(extract_songs_info(env, result.get("songs").unwrap().to_owned()).unwrap())
    } else {
        Ok(().into_lisp(env)?)
    }
}

pub async fn search_playlist<'a>(
    env: &'a Env,
    content: String,
    limit: i64,
    page: i64,
) -> EResult<EValue<'a>> {
    let api = get_api();
    let playlists = api
        .cloud_search(
            &content,
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
            Ok(().into_lisp(env)?)
        } else {
            Ok(extract_playlists_info(env, playlists.to_owned()).unwrap())
        }
    } else {
        Ok(().into_lisp(env)?)
    }
}

/// Search song or playlist.
/// SEARCH_CONTENT is the content you want to search,
/// If PLAYLISTP is a number, then search playlist, otherwise it's nil, search song.
/// LIMIT is the limitation of each search.
/// PAGE is the current search page.
#[defun]
#[tokio::main]
pub async fn search<'a>(
    search_content: String,
    playlistp: EValue<'a>,
    limit: i64,
    page: i64,
) -> EResult<EValue<'a>> {
    let env = playlistp.env;

    if playlistp.is_not_nil() {
        let result = search_playlist(env, search_content, limit, page).await;
        match result {
            Err(_) => Ok(().into_lisp(env)?),
            Ok(a) => Ok(a),
        }
    } else {
        let result = search_song(env, search_content, limit, page).await;
        match result {
            Err(_) => Ok(().into_lisp(env)?),
            Ok(a) => Ok(a),
        }
    }
}

// TODO: Notice format about the let result ....
/// Get the playlists of the user whose user id is UID.
#[defun]
#[tokio::main]
pub async fn user_playlist(env: &Env, uid: i64) -> EResult<EValue<'_>> {
    let api = get_api();
    let result =
        PlaylistsInfo::from_data(api.user_playlist(uid as usize, None).await.unwrap().data());
    let playlists = result.playlist.as_array().unwrap();

    if playlists.len() == 0 {
        Ok(().into_lisp(env)?)
    } else {
        // NOTE: Maybe now I'll not use `more` to know whether there're other results.
        Ok(
            // result.more
            extract_playlists_info(env, result.playlist).unwrap(),
        )
    }
}

/// Get lyrics of SID.
#[defun]
#[tokio::main]
pub async fn get_lyrics(env: &Env, sid: i64) -> EResult<EValue<'_>> {
    let api = get_api();
    let lyrics = LyricsInfo::from_data(api.lyric(sid as usize).await.unwrap().data());
    match lyrics.lrc {
        JValue::Null => Ok(().into_lisp(env)?),
        _ => Ok(env.list((
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
        ))?),
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
//     // let a = song_url(536622304).await.unwrap();
//     // println!("{}", a);
//     // song_url(10941904111).await;
// }

/// Get the song's comment by its ID and return it.
/// Warning: This function doesn't have side-effect.
#[defun]
#[tokio::main]
pub async fn get_comment(env: &Env, sid: i64, page_no: i64) -> EResult<EValue<'_>> {
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
    let result = result.data.get("comments").unwrap();
    let mut results = Vec::<EValue<'_>>::new();

    for i in result.as_array().unwrap().iter() {
        let user = i.get("user").unwrap();
        results.push(env.list((
            i.get("commentId").unwrap().as_i64().unwrap().to_owned(),
            i.get("content").unwrap().as_str().unwrap().to_owned(),
            user.get("nickname").unwrap().as_str().unwrap().to_owned(),
            user.get("avatarUrl").unwrap().as_str().unwrap().to_owned(),
        ))?);
    }
    Ok(env.list(&results)?)
}

/// The function to comment or reply CONTENT to a comment.
/// SID is the song's id.
/// When CID is non-nil, means to reply comment with cid(its id).
#[defun]
#[tokio::main]
pub async fn create_comment(sid: i64, content: String, cid: i64) -> EResult<bool> {
    let api = get_api();
    let result = if cid > 0 {
        api.comment_create(sid as usize, ncmapi::ResourceType::Song, &content)
            .await
            .unwrap()
            .deserialize_to_implict()
    } else {
        api.comment_re(
            sid as usize,
            ncmapi::ResourceType::Song,
            cid as usize,
            &content,
        )
        .await
        .unwrap()
        .deserialize_to_implict()
    };

    if result.code == 200 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Get songs' url with SID.
#[defun]
#[tokio::main]
pub async fn song_url(sid: i64) -> EResult<Option<String>> {
    let api = get_api();
    let url = api
        .song_url(&[sid as usize].to_vec())
        .await
        .unwrap()
        .deserialize_to_implict();

    if url.code != 200 {
        return Ok(None);
    }

    let result = url
        .data
        .as_array()
        .unwrap()
        .first()
        .unwrap()
        .get("url")
        .unwrap();
    match result {
        JValue::String(s) => Ok(Some(s.to_string())),
        _ => Ok(None),
    }
}
