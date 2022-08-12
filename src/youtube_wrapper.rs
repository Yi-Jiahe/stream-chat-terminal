use std::env;

use reqwest::header;

const API_KEY: &str = env!("API_KEY");

const BASE_URL: &str = "https://www.googleapis.com/youtube/v3";

pub struct Client {
    key: String,
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
            key: "".to_string(),
            client
        })
    }

    pub fn get_live_chat_messages(&self, live_chat_id: &str) -> Result<(), String> {
        let res = match &self.client.get(format!("{}/liveChat/messages?key={}&liveChatId={}&part=snippet", BASE_URL, API_KEY, live_chat_id))
            .send() {
                Ok(res) => dbg!(res),
                Err(err) => return Err(err.to_string())
            };

        Ok(())
    }
    fn get_stream_id(&self) {

    }
}