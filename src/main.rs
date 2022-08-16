use std::io;

use stream_chat_terminal::parser;

const MESSAGE_DELAY_MILLIS: i64 = 5000;

fn main() {
    println!("Video ID: ");

    let mut video_id = String::new();

    io::stdin()
        .read_line(&mut video_id)
        .expect("Failed to read line");

    parser::print_youtube_messages(&video_id, MESSAGE_DELAY_MILLIS);
}

