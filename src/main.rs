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
use std::default::Default;
extern crate tokio;
extern crate google_youtube3 as youtube3;
use youtube3::{Result, Error};
use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};

use stream_chat_terminal::config::Config;
use stream_chat_terminal::google_oauth::GoogleAuth;
use stream_chat_terminal::parser;

const MESSAGE_DELAY_MILLIS: i64 = 5000;

#[tokio::main]
async fn main() {
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

    let auth = GoogleAuth{
        retries: 3,
    };
    let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);

    println!("Video ID: ");
    let mut video_id = String::new();
    io::stdin()
        .read_line(&mut video_id)
        .expect("Failed to read line");

    let result = hub.videos().list(&vec!["snippet".into()])
        .add_id(&video_id)
        .doit().await;
    
    dbg!(result);

    if !args.post {
        // let result = hub.live_chat_messages().list(result[0], &vec!["snippet".into()])
        //      .profile_image_size(32)
        //      .max_results(28)
        //      .doit().await;
    } else {
    }
}
