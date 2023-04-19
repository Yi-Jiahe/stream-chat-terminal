use std::env;

use super::structs::*;

const GOOGLE_API_KEY: &str = env!("GOOGLE_API_KEY");

const BASE_URL: &str = "https://www.googleapis.com/youtube/v3";

pub struct Client<S: HttpSend = Sender> {
    client: reqwest::blocking::Client,
    sender: S,
    access_token: Option<String>,
}

impl Client<Sender> {
    pub fn new(access_token: Option<String>) -> Client<Sender> {
        Client {
            client: reqwest::blocking::Client::builder().build().unwrap(),
            sender: Sender,
            access_token: access_token,
        }
    }
}

impl<S: HttpSend> Client<S> {
    pub fn with_sender(sender: S, access_token: Option<String>) -> Client<S> {
        Client {
            client: reqwest::blocking::Client::new(),
            sender: sender,
            access_token: access_token,
        }
    }
}

impl Client {
    fn get(&self, url: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let req = if let Some(token) = &self.access_token {
            self.client
                .get(url)
                .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        } else {
            self.client.get(url)
        };

        let res = self.sender.send(req)?;

        return Ok(res);
    }

    pub fn get_stream_id(&self, video_id: &str) -> Result<String, String> {
        let url = format!(
            "{}/videos?key={}&id={}&part=liveStreamingDetails",
            BASE_URL, GOOGLE_API_KEY, video_id
        );
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
                "{}/liveChat/messages?key={}&liveChatId={}&part=snippet,authorDetails",
                BASE_URL, GOOGLE_API_KEY, live_chat_id
            )
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

    pub fn insert_live_chat_message(&self, live_chat_id: &str, message_text: &str) {
        let url = format!(
            "{}/liveChat/messages?part=snippet&key={}",
            BASE_URL, GOOGLE_API_KEY
        );
        if let Some(token) = &self.access_token {
            let body = LiveChatMessage {
                snippet: Some(LiveChatMessageSnippet {
                    r#type: Some("textMessageEvent".to_string()),
                    liveChatId: Some(live_chat_id.to_string()),
                    textMessageDetails: Some(LiveChatMessageSnippetTextMessageDetails {
                        messageText: message_text.to_string(),
                    }),
                    ..LiveChatMessageSnippet::default()
                }),
                ..LiveChatMessage::default()
            };

            let req = self
                .client
                .post(url)
                .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
                .json(&body);

            let res = self.sender.send(req).unwrap();

            if res.status() != reqwest::StatusCode::OK {
                println!(
                    "Non 200 response code: {}, Try refreshing your OAuth Token",
                    res.status()
                );
            }
        } else {
            println!("Please complete the OAuth flow to post messages");
        }
    }
}

pub trait HttpSend {
    fn send(
        &self,
        request: reqwest::blocking::RequestBuilder,
    ) -> Result<reqwest::blocking::Response, reqwest::Error>;
}

pub struct Sender;
impl HttpSend for Sender {
    fn send(
        &self,
        request: reqwest::blocking::RequestBuilder,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        Ok(request.send()?)
    }
}
