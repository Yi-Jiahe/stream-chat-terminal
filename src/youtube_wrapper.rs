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
struct LiveChatMessageListResponse {
    nextPageToken: String,
    pollingIntervalMillis: u64,
    pageInfo: PageInfo,
    items: Vec<LiveChatMessage>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct PageInfo {
    totalResults: i64,
    resultsPerPage: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct Video {
    liveStreamingDetails: Option<LiveStreamingDetails>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct LiveChatMessage {
    snippet: Option<LiveChatMessageSnippet>,
    authorDetails: Option<LiveChatMessageAuthorDetails>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct LiveChatMessageSnippet {
    displayMessage: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct LiveChatMessageAuthorDetails {
    displayName: String
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

    pub fn get_live_chat_messages(&self, live_chat_id: &str) -> Result<Vec<String>, String> {
        let res = match self.client.get(format!("{}/liveChat/messages?key={}&liveChatId={}&part=snippet,authorDetails", BASE_URL, API_KEY, live_chat_id))
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

        for message in body.items {
        
            let display_name = match message.authorDetails {
                Some(author_details) => author_details.displayName,
                None => return Err("Author details missing".to_string())
            };
            let display_message = match message.snippet {
                Some(snippet) => snippet.displayMessage,
                None => return Err("Snippet missing".to_string())
            };

            println!("{}: {}", display_name, display_message);
        }

        Ok(Vec::new())
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
