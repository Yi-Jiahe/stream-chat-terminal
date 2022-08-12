use std::io;

use stream_chat_terminal::youtube_wrapper::Client;

fn main() {
    println!("Hello, world!");

    let yt = match Client::new() {
        Ok(client) => client,
        Err(err) => panic!("{}", err.to_string())
    };

    let mut stream_id = String::new();

    io::stdin()
        .read_line(&mut stream_id)
        .expect("Failed to read line");

    let messages = match yt.get_live_chat_messages(&stream_id) {
        Ok(messages) => messages,
        Err(err) => panic!("{}", err.to_string())
    };
}

