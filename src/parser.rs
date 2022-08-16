use std::{thread, time};

use crate::youtube_wrapper::Client;

pub fn print_youtube_messages(video_id: &str) {
    let yt = match Client::new() {
        Ok(client) => client,
        Err(err) => panic!("{}", err.to_string())
    };

    let live_chat_id = match yt.get_stream_id(&video_id) {
        Ok(live_chat_id) => live_chat_id,
        Err(err) => panic!("{}", err.to_string())
    };

    let mut next_page_token: String = String::from("");

    loop {
        let body = match yt.get_live_chat_messages(&live_chat_id, &next_page_token) {
            Ok(body) => body,
            Err(err) => panic!("{}", err.to_string())
        };
    
        // println!("{}", body.pageInfo.totalResults);
        // println!("{}", body.items.len());
    
        for message in body.items {
            let display_name = match message.authorDetails {
                Some(author_details) => author_details.displayName,
                None => continue
            };
            let (published_at, display_message) = match message.snippet {
                Some(snippet) => (snippet.publishedAt, snippet.displayMessage),
                None => continue
            };
    
            println!("{} {}: {}", published_at, display_name, display_message);
        }

        next_page_token = body.nextPageToken.clone();

        // println!("{}", body.pollingIntervalMillis);
        thread::sleep(time::Duration::from_millis(body.pollingIntervalMillis));
    }
    
}