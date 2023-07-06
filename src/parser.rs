use std::{thread, time};

use ansi_term::Colour;
use chrono::{DateTime, Utc};

extern crate google_youtube3 as youtube3;
use youtube3::api::{LiveChatMessage};

pub fn print_youtube_messages(messages: Vec<LiveChatMessage>, delay_milliseconds: i64) {
    for message in messages {
        let (display_name, is_chat_sponsor) = match message.author_details {
            Some(author_details) => (
                match author_details.display_name {
                    Some(display_name) => display_name,
                    None => {
                        println!("Unable to retrieve display name");
                        continue},
                }, 
                match author_details.is_chat_sponsor {
                    Some(is_chat_sponsor) => is_chat_sponsor,
                    None => false,
                }),
            None => {
                println!("Unable to retrieve author details");
                continue
            },
        };
        let (published_at, display_message) = match message.snippet {
            Some(snippet) => (snippet.published_at, snippet.display_message),
            None => {
                println!("Unable to retrieve message details");
                continue
            },
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
        println!(
            "{}: {}",
            if is_chat_sponsor {
                Colour::Green.bold().paint(display_name)
            } else {
                display_name.into()
            },
            display_message.unwrap()
        );
    }
}
