use clap::Parser;

/// Bring stream chat to your terminal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Show config path
    #[clap(long, short, action)]
    config_path: bool,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    api_key: Option<String>,
}

impl ::std::default::Default for Config {
    fn default() -> Self { Self { api_key: None } }
}

use std::io;

use stream_chat_terminal::parser;
use stream_chat_terminal::youtube_wrapper::Client;

const MESSAGE_DELAY_MILLIS: i64 = 5000;

fn main() {
    let args = Args::parse();

    if args.config_path {
        let configuration_file_path = confy::get_configuration_file_path("stream-chat-terminal", None)
            .expect("Unable to get config file path");
        println!("{}", configuration_file_path.display();
        return;
    }

    let mut cfg: Config = confy::load("stream-chat-terminal", None).expect("Unable to load config");

    dbg!(&cfg);

    if cfg.api_key.is_none() {
        let mut api_key = String::new();

        println!("Please provide your YouTube API key: (None)");
        io::stdin()
            .read_line(&mut api_key)
            .expect("Failed to read line");

        api_key = api_key.trim().to_string();

        cfg.api_key = if api_key.is_empty() {
            println!("{}", api_key.is_empty());
            None
        } else {
            Some(api_key)
        };

        dbg!(&cfg);

        confy::store("my-app-name", None, &cfg).expect("Failed to store config");
    }

    let yt = match Client::new(cfg.api_key) {
        Ok(client) => client,
        Err(err) => panic!("{}", err.to_string()),
    };

    println!("Video ID: ");

    let mut video_id = String::new();

    io::stdin()
        .read_line(&mut video_id)
        .expect("Failed to read line");

    parser::print_youtube_messages(yt, &video_id, MESSAGE_DELAY_MILLIS);
}
