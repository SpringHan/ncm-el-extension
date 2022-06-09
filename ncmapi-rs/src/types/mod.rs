// This module was generated at https://transform.tools/json-to-rust-serde
// However, some fields of struct was stripped for concision.
pub type SearchSongResp = ResultResp<SearchResultSong>;
pub type SearchArtistResp = ResultResp<SearchResultArtist>;
pub type SearchPodcastResp = ResultResp<SearchResultPodcast>;
pub type SearchPlaylistResp = ResultResp<SearchResultPlaylist>;
pub type SearchAlbumResp = ResultResp<SearchResultAlbum>;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultResp<T> {
    pub code: usize,
    pub result: Option<T>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SearchResultSong {
    pub songs: Vec<Song>,
    pub has_more: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: usize,
    pub name: String,
    #[serde(alias = "ar")]
    pub artists: Vec<Artist>,
    #[serde(alias = "al")]
    pub album: Album,
    #[serde(alias = "dt")]
    pub duration: usize,
    pub fee: usize,
    #[serde(alias = "popularity")]
    pub pop: f32,
    // pub resource_state: bool,
    // pub publish_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: usize,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: usize,
    pub name: Option<String>,
    #[serde(default)]
    pub pic_url: String,
    pub pic: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastAudio {
    pub main_song: Song,
    pub dj: UserProfile,
    pub liked_count: usize,
    pub comment_count: usize,
}

/// User created podcasts
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPodcastsResp {
    pub code: usize,
    #[serde(default)]
    pub dj_radios: Vec<Podcast>,
    #[serde(default)]
    pub has_more: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastAudiosResp {
    pub code: usize,
    #[serde(default)]
    pub programs: Vec<PodcastAudio>,
    #[serde(default)]
    pub more: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub user_id: usize,
    pub nickname: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAccountResp {
    pub code: usize,
    pub profile: Option<UserProfile>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserPlaylistResp {
    pub code: usize,
    #[serde(default)]
    pub playlist: Vec<Playlist>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlaylistDetailResp {
    pub code: usize,
    pub playlist: Option<PlaylistDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistDetail {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tracks: Vec<Song>,
    #[serde(default)]
    pub track_ids: Vec<Id>,
    pub user_id: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Id {
    pub id: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SongUrlResp {
    pub code: usize,
    #[serde(default)]
    pub data: Vec<SongUrl>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SongUrl {
    pub id: usize,
    pub url: String,
    pub br: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCloudResp {
    pub code: usize,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub data: Vec<CloudSongMeta>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudSongMeta {
    pub simple_song: Song,
    pub song_id: usize,
    pub song_name: String,
    pub add_time: i128,
    pub file_size: usize,
    pub bitrate: usize,
    pub file_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedSongs {
    #[serde(default)]
    pub daily_songs: Vec<Song>,
    #[serde(default)]
    pub order_songs: Vec<Song>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecommendedSongsResp {
    pub code: usize,
    pub data: RecommendedSongs,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub user: UserProfile,
    #[serde(default)]
    pub content: String,
    pub time: u64,
    pub liked_count: usize,
    pub liked: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceComments {
    #[serde(default)]
    pub comments: Vec<Comment>,
    pub total_count: usize,
    pub has_more: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceCommentsResp {
    pub code: usize,
    pub data: ResourceComments,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotCommentsResp {
    pub code: usize,
    #[serde(default)]
    pub hot_comments: Vec<Comment>,
    pub has_more: bool,
    pub total: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricResp {
    pub code: usize,
    pub sgc: bool,
    pub sfy: bool,
    pub qfy: bool,
    pub lrc: Option<Lyric>,
    pub klyric: Option<Lyric>,
    pub tlyric: Option<Lyric>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lyric {
    #[serde(default)]
    pub version: usize,
    #[serde(default)]
    pub lyric: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalFmResp {
    pub code: usize,
    #[serde(default)]
    pub data: Vec<Song>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendedPlaylistsResp {
    pub code: usize,
    #[serde(default)]
    pub recommend: Vec<Playlist>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimiSongsResp {
    pub code: usize,
    #[serde(default)]
    pub songs: Vec<Song>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSongsResp {
    pub code: usize,
    #[serde(default)]
    pub songs: Vec<Song>,
    #[serde(default)]
    pub more: bool,
    #[serde(default)]
    pub total: usize,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistSublistResp {
    pub code: usize,
    #[serde(default)]
    pub data: Vec<Artist>,
    #[serde(default)]
    pub has_more: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Podcast {
    pub id: usize,
    pub name: String,
    pub desc: String,
    pub sub_count: usize,
    pub category: String,
    pub dj: UserProfile,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultArtist {
    #[serde(default)]
    pub artists: Vec<Artist>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultPodcast {
    #[serde(default)]
    pub dj_radios: Vec<Podcast>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultPlaylist {
    #[serde(default)]
    pub playlists: Vec<Playlist>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultAlbum {
    #[serde(default)]
    pub albums: Vec<Album>,
}
