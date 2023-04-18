use std::env;

const GOOGLE_API_KEY: &str = env!("GOOGLE_API_KEY");

const BASE_URL: &str = "https://www.googleapis.com/youtube/v3";

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
enum Response {
    #[serde(rename = "youtube#videoListResponse")]
    VideoList(VideoListResponse),
    #[serde(rename = "youtube#liveChatMessageListResponse")]
    LiveChatMessageList(LiveChatMessageListResponse),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct VideoListResponse {
    items: Vec<Video>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LiveChatMessageListResponse {
    pub nextPageToken: String,
    pub pollingIntervalMillis: u64,
    pub pageInfo: PageInfo,
    pub items: Vec<LiveChatMessage>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct PageInfo {
    pub totalResults: i64,
    pub resultsPerPage: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct Video {
    liveStreamingDetails: Option<LiveStreamingDetails>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct LiveChatMessage {
    pub snippet: Option<LiveChatMessageSnippet>,
    pub authorDetails: Option<LiveChatMessageAuthorDetails>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct LiveChatMessageSnippet {
    pub publishedAt: String,
    pub displayMessage: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct LiveChatMessageAuthorDetails {
    pub displayName: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct LiveStreamingDetails {
    activeLiveChatId: String,
}

pub struct Client {
    client: reqwest::blocking::Client,
    access_token: Option<String>,
}

impl Client {
    pub fn new(access_token: Option<String>) -> Result<Client, String> {
        Ok(
            Client {
                client: reqwest::blocking::Client::builder().build().unwrap(),
                access_token: access_token
            }
        )
    }

    fn get(&self, url: String) -> Result<reqwest::blocking::Response, reqwest::Error>{
        let req = if let Some(token) = &self.access_token {
            self.client.get(url).header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        } else {
            self.client.get(url)
        };

        let res = req.send();

        return res
    }

    pub fn get_stream_id(&self, video_id: &str) -> Result<String, String> {
        let url = format!("{}/videos?key={}&id={}&part=liveStreamingDetails", BASE_URL, GOOGLE_API_KEY, video_id);
        let res = self.get(url).unwrap();

        if !res.status().is_success() {
            return Err("Invalid response".to_string());
        }

        let body = match res.json::<Response>() {
            Ok(body) => match body {
                Response::VideoList(body) => body,
                _ => return Err("Wrong response type".to_string()),
            },
            Err(err) => return Err(err.to_string()),
        };

        let video = match body.items.first() {
            Some(video) => video.clone(),
            None => return Err("No videos found".to_string()),
        };

        match video.liveStreamingDetails {
            Some(live_streaming_details) => {
                return Ok(live_streaming_details.activeLiveChatId.clone())
            }
            None => return Err("No live streaming details".to_string()),
        }
    }

    pub fn get_live_chat_messages(
        &self,
        live_chat_id: &str,
        next_page_token: &str,
    ) -> Result<LiveChatMessageListResponse, String> {
        let url = if next_page_token == "" {
            format!(
                "{}/liveChat/messages?key={}&liveChatId={}&part=snippet,authorDetails", BASE_URL, GOOGLE_API_KEY, live_chat_id )
        } else {
            format!("{}/liveChat/messages?key={}&liveChatId={}&page_token={}&part=snippet,authorDetails", BASE_URL, GOOGLE_API_KEY, live_chat_id, next_page_token)
        };

        let res = match self.get(url) {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let body = match res.json::<Response>() {
            Ok(body) => match body {
                Response::LiveChatMessageList(body) => body,
                _ => return Err("Wrong response type".to_string()),
            },
            Err(err) => return Err(err.to_string()),
        };

        Ok(body)
    }
}
