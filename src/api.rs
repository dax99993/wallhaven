use std::env;
use crate::args::CLICommands;

const ENV_API_KEY: &str = "WALLHAVEN_API_KEY";
pub const BASE_URL: &str = "https://wallhaven.cc/api/v1";


pub fn get_key() -> Result<String, String> {
    let v = env::var(ENV_API_KEY)
        .map_err(|_| format!("Cannot read API key"))?;

    return Ok(v);
}


// ------------------------------------------------------------
// Api response types
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "data")]
    pub data: Vec<Wallpaper>,
    #[serde(rename = "meta")]
    pub meta: WallpaperMeta,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "")]
pub struct Wallpaper {
    id: String,
    url: String,
    short_url: String,
    views: i32,
    favorites: i32,
    source: String,
    purity: String,
    category: String,
    dimension_x: i32,
    dimension_y: i32,
    resolution: String,
    ratio: String,
    file_size: i32,
    file_type: String,
    created_at: String,
    colors: Vec<String>,
    path: String,
    thumbs: Thumbs,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thumbs {
    large: String,
    original: String,
    small: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "")]
pub struct WallpaperMeta {
    current_page: i32,
    last_page: i32,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    per_page: i32, //Should be int,idk why its string even when api guide defines as int
    total: i32,
    query: MetaQuery,
    seed: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MetaQuery {
    Query(Option<String>),
    Querytag{id: i32, tag: Option<String>}
}



#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "")]
pub struct WallpaperInfoResponse{
    #[serde(rename = "data")]
    pub data: WallpaperInfo,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "")]
pub struct WallpaperInfo {
    pub id: String,
    pub url: String,
    pub short_url: String,
    pub uploader: Uploader,
    pub views: i32,
    pub favorites: i32,
    pub source: String,
    pub purity: String,
    pub category: String,
    pub dimension_x: i32,
    pub dimension_y: i32,
    pub resolution: String,
    pub ratio: String,
    pub file_size: i32,
    pub file_type: String,
    pub created_at: String,
    pub colors: Vec<String>,
    pub path: String,
    pub thumbs: Thumbs,
    pub tags: Vec<Tag>
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Uploader {
    pub username: String,
    pub group: String,
    pub avatar: Avatar,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Avatar {
    #[serde(rename = "200px")]
    pub _200px: String,
    #[serde(rename = "128px")]
    pub _128px: String,
    #[serde(rename = "32px")]
    pub _32px: String,
    #[serde(rename = "20px")]
    pub _20px: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "")]
pub struct TagResponse {
    #[serde(rename = "data")]
    pub data: Tag,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "")]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub alias: String,
    pub category_id: i32,
    pub category: String,
    pub purity: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "")]
pub struct UserSettingsResponse {
    #[serde(rename = "data")]
    pub data: UserSettings,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "")]
pub struct UserSettings {
    thumb_size: String,
    per_page: String,
    purity: Vec<String>,
    categories: Vec<String>,
    resolutions: Vec<String>,
    aspect_ratios: Vec<String>,
    toplist_range: String,
    tag_blacklist: Vec<String>,
    user_blacklist: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "")]
pub struct UserCollectionsResponse {
    #[serde(rename = "data")]
    pub data: Vec<UserCollections >,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "")]
pub struct UserCollections {
    id: i32,
    label: String,
    views: i32,
    public: i32,
    count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "")]
pub struct ErrorResponse {
    error: String,
}



pub(crate) trait Url {
    fn to_url(&self, base_url: &str) -> String;
}


use futures::TryFutureExt;
use futures::stream::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub enum WallhavenClientError {
    RequestError(String),
    DecodeError(String),
    WriteError(String),
}

impl std::fmt::Display for WallhavenClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DecodeError(e) => {
                write!(f, "Decode Error - {}", e)
            },
            Self::WriteError(e) => {
                write!(f, "Write Error - {}", e)
            },
            Self::RequestError(e) => {
                write!(f, "Request Error - {}", e)
            },
        }
    }
}

impl std::error::Error for WallhavenClientError {}


#[derive(Debug)]
pub struct WallhavenClient {
    http_client: reqwest::Client,
    commands: CLICommands,
}

impl WallhavenClient {
    pub fn new(commands: CLICommands) -> Result<Self, String> {
        let api_key = get_key();

        /* Create http client */
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));
        headers.insert(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json"));
        if let Ok(k) = api_key {
            let header_api_value = reqwest::header::HeaderValue::from_str(&k).map_err(|_| String::from("header error"))?;
            headers.insert("X-API-KEY", header_api_value);
        }

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .map_err(|e| e.to_string())?;

        Ok(
            Self {
                http_client: client,
                commands,
            }
        )
    }

    pub async fn execute(&self) -> Result<String, WallhavenClientError> {
        let resp = match &self.commands {
            CLICommands::Search(s) => {
                let res = self.request(s.to_url(BASE_URL))
                    .await?;

                // Check if we got bad status response and return it
                if let Ok(r) = serde_json::from_str::<ErrorResponse>(&res) {
                    let error_response = serde_json::to_string(&r)
                            .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?;
                    return Ok(error_response);
                }

                // Check if response has the structure as described in api guide
                let searchresp: SearchResponse = serde_json::from_str(&res)
                    .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?;


                //download wallpapers
                if let Some(path) = s.path.clone() {
                    for w in searchresp.data {
                        let path = path.clone();
                        let wallpaper_path = std::path::PathBuf::from(path);
                        // Should I do a safe check, even if I know the api wallpaper response
                        // has such format?
                        if let Some(image_name) = w.path.split("/").last() {
                            let file_path = wallpaper_path.join(image_name);

                            self.download_image(&w.path, &file_path).await?
                        }
                    }

                    format!("")
                } else {
                    serde_json::to_string(&searchresp)
                    .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?
                }
            }, 
            CLICommands::WallpaperInfo(w) => {
                let res = self.request(w.to_url(BASE_URL))
                    .await?;

                if let Ok(r) = serde_json::from_str::<ErrorResponse>(&res) {
                    let error_response = serde_json::to_string(&r)
                            .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?;
                    return Ok(error_response);
                }

                // Parse to data structure to check it keeps the same structure as described in api
                // guide
                let wallpaperinfo: WallpaperInfoResponse = serde_json::from_str(&res)
                    .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?;

                serde_json::to_string(&wallpaperinfo)
                    .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?
            },
            CLICommands::TagInfo(t) => {
                let res = self.request(t.to_url(BASE_URL))
                    .await?;

                if let Ok(r) = serde_json::from_str::<ErrorResponse>(&res) {
                    let error_response = serde_json::to_string(&r)
                            .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?;
                    return Ok(error_response);
                }

                let taginfo: TagResponse = serde_json::from_str(&res)
                    .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?;

                serde_json::to_string(&taginfo)
                    .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?
            },
            CLICommands::UserSettings(us) => {
                let res = self.request(us.to_url(BASE_URL))
                    .await?;

                if let Ok(r) = serde_json::from_str::<ErrorResponse>(&res) {
                    let error_response = serde_json::to_string(&r)
                            .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?;
                    return Ok(error_response);
                }

                let usersettings: UserSettingsResponse = serde_json::from_str(&res)
                    .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?;

                serde_json::to_string(&usersettings)
                    .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?
            },
            CLICommands::UserCollections(uc) => {
                let res = self.request(uc.to_url(BASE_URL))
                    .await?;

                if let Ok(r) = serde_json::from_str::<ErrorResponse>(&res) {
                    let error_response = serde_json::to_string(&r)
                            .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?;
                    return Ok(error_response);
                }

                let usercollections: UserCollectionsResponse = serde_json::from_str(&res)
                    .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?;


                serde_json::to_string(&usercollections)
                    .map_err(|e| WallhavenClientError::DecodeError(e.to_string()))?
            }
        };

        Ok(resp)
    }

    pub async fn request(&self, url: String) -> Result<String, WallhavenClientError> {
        let response = self.http_client
            .get(url)
            .send()
            .await
            .map_err(|e| WallhavenClientError::RequestError(e.to_string()))?;

        match response.text().await {
            Ok(r) => {
                Ok(r)
            },
            Err(e) => {
                Err(
                    WallhavenClientError::DecodeError(e.to_string())
                )
            }
        }

    }

    pub async fn download_image(&self, url: &str, path: &std::path::PathBuf) -> Result<(), WallhavenClientError> {
        // Reqwest setup
        let res = self.http_client
            .get(url)
            .send()
            .await
            .map_err(|e| WallhavenClientError::RequestError(e.to_string()))?;

        //println!("{:#?}", res);

        // Get information for bar
        let total_size = res
            .content_length()
            .ok_or(format!("Failed to get content length from '{}'", &url))
            .map_err(|e| WallhavenClientError::RequestError(e))?;

        // Indicatif setup
        let pb = ProgressBar::new(total_size);
        let style = ProgressStyle::with_template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .unwrap()
            .progress_chars("#>-");
        pb.set_style(style);
        pb.set_message(format!("Downloading {}", url));

        // Create file path
        let file_path = std::path::Path::new(path);
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)
            .await
            .map_err(|e| WallhavenClientError::WriteError(
                    format!("Failed to create file - {}", e.to_string()))
                )?;


        // Write file
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item
                .or(Err(
                    WallhavenClientError::RequestError(format!("Error while downloading file"))
                    ))?;

            file.write(&chunk)
                .map_err(|e| WallhavenClientError::WriteError(format!("Error while writing to file - {}", e.to_string())))
                .await?;

            let new = u64::min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }

        pb.finish_with_message(format!("Downloaded {}", url));

        Ok(())
    }
}
