use std::{thread, time};

use ansi_term::Colour;
use chrono::{DateTime, Utc};

use crate::youtube_wrapper;

pub fn print_message(message: youtube_wrapper::LiveChatMessage, delay_milliseconds: i64) {
    let (display_name, is_chat_sponsor) = match message.authorDetails {
        Some(author_details) => (author_details.displayName, author_details.isChatSponsor),
        None => {
            println!("{}", Colour::Red.bold().paint("Missing author details"));
            return;
        }
    };
    let (published_at, display_message) = match message.snippet {
        Some(snippet) => (snippet.publishedAt, snippet.displayMessage),
        None => {
            println!(
                "{}: {}",
                if is_chat_sponsor {
                    Colour::Green.bold().paint(display_name)
                } else {
                    display_name.into()
                },
                Colour::Red.bold().paint("Missing message snippet")
            );
            return;
        }
    };

    // Delay print to maintain message position when printed
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
