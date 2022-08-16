use std::io;

use stream_chat_terminal::youtube_wrapper::Client;

fn main() {
    println!("Hello, world!");

    let yt = match Client::new() {
        Ok(client) => client,
        Err(err) => panic!("{}", err.to_string())
    };

    let mut video_id = String::new();

    io::stdin()
        .read_line(&mut video_id)
        .expect("Failed to read line");

    let live_chat_id = match yt.get_stream_id(&video_id) {
        Ok(live_chat_id) => live_chat_id,
        Err(err) => panic!("{}", err.to_string())
    };

    let messages = match yt.get_live_chat_messages(&live_chat_id) {
        Ok(messages) => messages,
        Err(err) => panic!("{}", err.to_string())
    };

    for message in messages {
        println!("{}", message);
    }
}

