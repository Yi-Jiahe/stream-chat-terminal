use clap::Parser;

/// Bring stream chat to your terminal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Show config path
    #[clap(long, short, action)]
    config_path: bool,

    #[clap(long, short, action)]
    post: bool,
}

use chrono::{DateTime, Utc};
use std::io;
use std::{thread, time};
extern crate google_clis_common;
extern crate google_youtube3 as youtube3;
extern crate tokio;
use youtube3::{hyper, hyper_rustls, oauth2, YouTube};

use stream_chat_terminal::parser;

const MESSAGE_DELAY_MILLIS: i64 = 5000;

#[tokio::main]
async fn main() {
    let configuration_file_path = confy::get_configuration_file_path("stream-chat-terminal", None)
        .expect("Unable to get config file path");
    let config_dir = configuration_file_path.parent().unwrap().to_str().unwrap();

    let args = Args::parse();

    if args.config_path {
        println!("{}", configuration_file_path.display());
        return;
    }

    let client = hyper::Client::builder().build(
        hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build(),
    );

    // Client ID and secret are not treated as secret for desktop applications
    // https://developers.google.com/identity/protocols/oauth2#installed
    let google_application_secret = google_clis_common::application_secret_from_directory(&config_dir, "youtube3-secret.json",
"{\"installed\":{\"client_id\":\"294311023223-9etdka9ubk21tshtp8modlfrapb08dvi.apps.googleusercontent.com\",\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"token_uri\":\"https://oauth2.googleapis.com/token\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\",\"client_secret\":\"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\",\"redirect_uris\":[\"http://localhost\"]}}"                                             
    ).unwrap();
    let auth = oauth2::InstalledFlowAuthenticator::with_client(
        google_application_secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        client.clone(),
    )
    .persist_tokens_to_disk(format!("{}/youtube3", config_dir))
    .build()
    .await
    .unwrap();

    let hub = YouTube::new(client, auth);

    println!("Video ID: ");
    let mut video_id = String::new();
    io::stdin()
        .read_line(&mut video_id)
        .expect("Failed to read line");

    let (_, video_list_response) = hub
        .videos()
        .list(&vec!["liveStreamingDetails".into()])
        .add_id(video_id.trim())
        .doit()
        .await
        .unwrap();
    let video = video_list_response.items.unwrap().pop().unwrap();
    let active_live_chat_id = video
        .live_streaming_details
        .unwrap()
        .active_live_chat_id
        .unwrap();

    dbg!(&active_live_chat_id);

    if !args.post {
        let mut next_page_token: String = String::from("");

        loop {
            let request_dt: DateTime<Utc> = Utc::now();

            let (_, live_chat_message_list_response) = hub
                .live_chat_messages()
                .list(
                    &active_live_chat_id,
                    &vec!["snippet".into(), "authorDetails".into()],
                )
                .profile_image_size(32)
                .max_results(28)
                .page_token(&next_page_token)
                .doit()
                .await
                .unwrap();

            let live_chat_messages = live_chat_message_list_response.items.unwrap();

            parser::print_youtube_messages(live_chat_messages, MESSAGE_DELAY_MILLIS);

            next_page_token = live_chat_message_list_response.next_page_token.unwrap();

            let time_elapsed = Utc::now() - request_dt;
            let time_to_wait = chrono::Duration::milliseconds(
                live_chat_message_list_response
                    .polling_interval_millis
                    .unwrap()
                    .into(),
            );
            if time_elapsed < time_to_wait {
                thread::sleep(time::Duration::from_millis(
                    (time_to_wait - time_elapsed)
                        .num_milliseconds()
                        .try_into()
                        .unwrap(),
                ));
            }
        }
    } else {
        loop {
            let mut display_message = String::new();
            io::stdin()
                .read_line(&mut display_message)
                .expect("Failed to read line");

            let message = youtube3::api::LiveChatMessage {
                snippet: Some(youtube3::api::LiveChatMessageSnippet {
                    type_: Some("textMessageEvent".into()),
                    live_chat_id: Some(active_live_chat_id.clone()),
                    text_message_details: Some(youtube3::api::LiveChatTextMessageDetails {
                        message_text: Some(display_message),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            };

            hub.live_chat_messages()
                .insert(message)
                .doit()
                .await
                .unwrap();
        }
    }
}
