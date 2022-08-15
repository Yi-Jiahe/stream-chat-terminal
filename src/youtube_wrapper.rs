use std::env;

use reqwest::header;
use serde::Deserialize;

const API_KEY: &str = env!("API_KEY");

const BASE_URL: &str = "https://www.googleapis.com/youtube/v3";

#[derive(Deserialize, Debug)]
struct ResponseBody {
    items: Vec<Item>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct Item {
    liveStreamingDetails: Option<LiveStreamingDetails>
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
        let oauth2_token = ""; 
        let header_value = format!("Bearer {}", oauth2_token);

        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static(Box::leak(header_value.into_boxed_str())));

        let client = match reqwest::blocking::Client::builder()
            .build(){
                Ok(client) => client,
                Err(err) => return Err(err.to_string())
            };

        Ok(Client{
            client
        })
    }

    pub fn get_live_chat_messages(&self, video_id: &str) -> Result<Vec<String>, String> {
        let live_chat_id = match self.get_stream_id(video_id) {
            Ok(live_chat_id) => live_chat_id,
            Err(err) => return Err(err.to_string())
        };

        let res = match dbg!(self.client.get(format!("{}/liveChat/messages?key={}&liveChatId={}&part=snippet", BASE_URL, API_KEY, live_chat_id))
            .send()) {
                Ok(res) => res,
                Err(err) => return Err(err.to_string())
            };

        Ok(Vec::new())
    }

    fn get_stream_id(&self, video_id: &str) -> Result<String, String> {
        let res = match self.client.get(format!("{}/videos?key={}&id={}&part=liveStreamingDetails", BASE_URL, API_KEY, video_id))
            .send() {
                Ok(res) => res,
                Err(err) => return Err(err.to_string()), 
            };

        let body = match res.json::<ResponseBody>() {
            Ok(body) => body,
            Err(err) => return Err(err.to_string()), 
        };

        let item = body.items[0].clone();

        match item.liveStreamingDetails {
            Some(live_streaming_details) => return Ok(live_streaming_details.activeLiveChatId.clone()),
            None => return Err("No live streaming details".to_string())
        }
    }
}