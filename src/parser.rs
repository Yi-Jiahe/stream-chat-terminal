use chrono::{DateTime, Local, Utc};
use std::{thread, time};

use crate::youtube_wrapper::Client;

pub fn print_youtube_messages(yt: Client, live_chat_id: &str, delay_milliseconds: i64) {
    let mut next_page_token: String = String::from("");

    loop {
        let body = match yt.get_live_chat_messages(&live_chat_id, &next_page_token) {
            Ok(body) => body,
            Err(err) => panic!("{}", err.to_string()),
        };

        let request_dt: DateTime<Utc> = Utc::now();

        // println!("{}", body.pageInfo.totalResults);
        // println!("{}", body.items.len());

        for message in body.items {
            let display_name = match message.authorDetails {
                Some(author_details) => author_details.displayName,
                None => continue,
            };
            let (published_at, display_message) = match message.snippet {
                Some(snippet) => (snippet.publishedAt, snippet.displayMessage),
                None => continue,
            };

            let published_dt = match DateTime::parse_from_rfc3339(&published_at.unwrap()) {
                Ok(dt) => dt,
                Err(err) => panic!("{}", err),
            };
            let time_to_wait = (published_dt + chrono::Duration::milliseconds(delay_milliseconds))
                .with_timezone(&Utc)
                - Utc::now();
            if time_to_wait > chrono::Duration::milliseconds(0) {
                thread::sleep(time::Duration::from_millis(
                    time_to_wait.num_milliseconds().try_into().unwrap(),
                ));
            }

            // println!("{} {}: {}", published_dt.with_timezone(&Local).format("%H:%M:%S").to_string(), display_name, display_message);
            println!("{}: {}", display_name, display_message.unwrap());
        }

        next_page_token = body.nextPageToken.clone();

        // println!("{}", body.pollingIntervalMillis);
        let now: DateTime<Utc> = Utc::now();
        let time_elapsed = now - request_dt;
        let time_to_wait =
            chrono::Duration::milliseconds(body.pollingIntervalMillis.try_into().unwrap());
        if time_elapsed < time_to_wait {
            thread::sleep(time::Duration::from_millis(
                (time_to_wait - time_elapsed)
                    .num_milliseconds()
                    .try_into()
                    .unwrap(),
            ));
        }
    }
}
