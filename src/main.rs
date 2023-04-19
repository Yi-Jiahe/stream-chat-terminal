use clap::Parser;

/// Bring stream chat to your terminal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Show config path
    #[clap(long, short, action)]
    config_path: bool,

    /// Set config
    #[clap(long, short, action)]
    set_config: bool,

    #[clap(long, short, action)]
    google_oauth: bool,

    #[clap(long, short, action)]
    post: bool,
}

use std::io;
use std::{thread, time};

use chrono::{DateTime, Utc};

use stream_chat_terminal::config::Config;
use stream_chat_terminal::google_oauth;
use stream_chat_terminal::parser;
use stream_chat_terminal::youtube::client::Client;

const MESSAGE_DELAY_MILLIS: i64 = 5000;

fn main() {
    let args = Args::parse();

    if args.config_path {
        let configuration_file_path =
            confy::get_configuration_file_path("stream-chat-terminal", None)
                .expect("Unable to get config file path");
        println!("{}", configuration_file_path.display());
        return;
    }

    let mut cfg: Config = confy::load("stream-chat-terminal", None).expect("Unable to load config");

    dbg!(&cfg);

    if args.google_oauth {
        google_oauth::oauth_flow(&mut cfg);
    }

    let yt = Client::new(cfg.google_access_token);

    println!("Video ID: ");

    let mut video_id = String::new();

    io::stdin()
        .read_line(&mut video_id)
        .expect("Failed to read line");

    let live_chat_id = match yt.get_stream_id(&video_id) {
        Ok(live_chat_id) => live_chat_id,
        Err(err) => panic!("{}", err.to_string()),
    };

    if !args.post {
        poll_live_chat_messages(yt, &live_chat_id, MESSAGE_DELAY_MILLIS);
    } else {
        loop {
            let mut message = String::new();

            io::stdin()
                .read_line(&mut message)
                .expect("Failed to read line");

            yt.insert_live_chat_message(&live_chat_id, &message);
        }
    }
}

pub fn poll_live_chat_messages(yt: Client, live_chat_id: &str, delay_milliseconds: i64) {
    let mut next_page_token: String = String::from("");

    loop {
        let body = match yt.get_live_chat_messages(&live_chat_id, &next_page_token) {
            Ok(body) => body,
            Err(err) => panic!("{}", err.to_string()),
        };

        let request_dt: DateTime<Utc> = Utc::now();

        for message in body.items {
            parser::print_message(message, delay_milliseconds)
        }

        next_page_token = body.nextPageToken.clone();

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
