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
}

use std::io;

use stream_chat_terminal::config::Config;
use stream_chat_terminal::google_oauth;
use stream_chat_terminal::parser;
use stream_chat_terminal::youtube_wrapper::Client;

const MESSAGE_DELAY_MILLIS: i64 = 5000;

fn main() {
    let args = Args::parse();

    if args.config_path {
        let configuration_file_path = confy::get_configuration_file_path("stream-chat-terminal", None)
            .expect("Unable to get config file path");
        println!("{}", configuration_file_path.display());
        return;
    }

    let mut cfg: Config = confy::load("stream-chat-terminal", None).expect("Unable to load config");

    dbg!(&cfg);

    if args.google_oauth {
        google_oauth::oauth_flow(&mut cfg);
    }

    let yt = match Client::new(cfg.google_access_token) {
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
