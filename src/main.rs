use std::io;

use stream_chat_terminal::parser;

fn main() {
    println!("Hello, world!");

    let mut video_id = String::new();

    io::stdin()
        .read_line(&mut video_id)
        .expect("Failed to read line");

    parser::print_youtube_messages(&video_id);
 
}

