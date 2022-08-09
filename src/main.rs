use stream_chat_terminal::youtube_wrapper::Client;

fn main() {
    println!("Hello, world!");

    let yt = match Client::new() {
        Ok(client) => client,
        Err(err) => panic!("{}", err.to_string())
    };

    let messages = match yt.get_live_chat_messages("asdf") {
        Ok(messages) => messages,
        Err(err) => panic!("{}", err.to_string())
    };
}
