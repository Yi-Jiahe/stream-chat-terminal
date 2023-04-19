use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "kind")]
pub enum Response {
    #[serde(rename = "youtube#videoListResponse")]
    VideoList(VideoListResponse),
    #[serde(rename = "youtube#liveChatMessageListResponse")]
    LiveChatMessageList(LiveChatMessageListResponse),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct VideoListResponse {
    pub items: Vec<Video>,
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
pub struct Video {
    pub liveStreamingDetails: Option<LiveStreamingDetails>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct LiveChatMessage {
    pub snippet: Option<LiveChatMessageSnippet>,
    pub authorDetails: Option<LiveChatMessageAuthorDetails>,
}

#[allow(non_snake_case)]
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct LiveChatMessageSnippet {
    pub r#type: Option<String>,
    pub liveChatId: Option<String>,
    pub publishedAt: Option<String>,
    pub displayMessage: Option<String>,
    pub textMessageDetails: Option<LiveChatMessageSnippetTextMessageDetails>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiveChatMessageSnippetTextMessageDetails {
    pub messageText: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiveChatMessageAuthorDetails {
    pub displayName: String,
    pub isChatSponsor: bool,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct LiveStreamingDetails {
    pub activeLiveChatId: String,
}
