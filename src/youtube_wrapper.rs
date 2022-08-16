use std::env;

use serde::Deserialize;

const API_KEY: &str = env!("API_KEY");

const BASE_URL: &str = "https://www.googleapis.com/youtube/v3";

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
enum Response {
    #[serde(rename = "youtube#videoListResponse")]
    VideoList(VideoListResponse),
    #[serde(rename = "youtube#liveChatMessageListResponse")]
    LiveChatMessageList(LiveChatMessageListResponse)
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct VideoListResponse {
    items: Vec<Video>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct LiveChatMessageListResponse {
    pub nextPageToken: String,
    pub pollingIntervalMillis: u64,
    pub pageInfo: PageInfo,
    pub items: Vec<LiveChatMessage>
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
    liveStreamingDetails: Option<LiveStreamingDetails>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct LiveChatMessage {
    pub snippet: Option<LiveChatMessageSnippet>,
    pub authorDetails: Option<LiveChatMessageAuthorDetails>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct LiveChatMessageSnippet {
    pub publishedAt: String,
    pub displayMessage: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct LiveChatMessageAuthorDetails {
    pub displayName: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct LiveStreamingDetails {
    activeLiveChatId: String
}

pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Result<Client, String> {
        let client = match reqwest::blocking::Client::builder()
            .build(){
                Ok(client) => client,
                Err(err) => return Err(err.to_string())
            };

        Ok(Client{
            client
        })
    }

    pub fn get_live_chat_messages(&self, live_chat_id: &str, next_page_token: &str) -> Result<LiveChatMessageListResponse, String> {
        let url = if next_page_token == "" {
            format!("{}/liveChat/messages?key={}&liveChatId={}&part=snippet,authorDetails", BASE_URL, API_KEY, live_chat_id)
        } else {
            format!("{}/liveChat/messages?key={}&liveChatId={}&page_token={}&part=snippet,authorDetails", BASE_URL, API_KEY, live_chat_id, next_page_token)
        };

        let res = match self.client.get(url)
            .send() {
                Ok(res) => res,
                Err(err) => return Err(err.to_string())
            };

        let body = match res.json::<Response>() {
            Ok(body) => match body {
                Response::LiveChatMessageList(body) => body,
                _ => return Err("Wrong response type".to_string())
            },
            Err(err) => return Err(err.to_string()), 
        };

        Ok(body)
    }

    pub fn get_stream_id(&self, video_id: &str) -> Result<String, String> {
        let res = match self.client.get(format!("{}/videos?key={}&id={}&part=liveStreamingDetails", BASE_URL, API_KEY, video_id))
            .send() {
                Ok(res) => res,
                Err(err) => return Err(err.to_string()), 
            };
        
        if !res.status().is_success() {
            return Err("Invalid response".to_string())
        }

        let body = match res.json::<Response>() {
            Ok(body) => match body {
                Response::VideoList(body) => body,
                _ => return Err("Wrong response type".to_string())
            },
            Err(err) => return Err(err.to_string()), 
        };

        let video = body.items[0].clone();

        match video.liveStreamingDetails {
            Some(live_streaming_details) => return Ok(live_streaming_details.activeLiveChatId.clone()),
            None => return Err("No live streaming details".to_string())
        }
    }
}
