# stream-chat-terminal

Display your stream chat in the terminal!

Polls YouTube chat list API to stream chat to your terminal.

## To-Do

### YouTube

- [ ] Switch to [google-youtube3](https://crates.io/crates/google-youtube3) crate for youtube functions.

I don't know why I didn't use it from the start.
Seems like it doesn't support requests w/o OAuth, i.e. the user needs to sign in even to perform API requests that do not require an OAuth token.

[google-youtube3-cli](https://crates.io/crates/google-youtube3-cli) is the cli generated from the wrapper. [Src](https://github.com/Byron/google-apis-rs/tree/main/gen/youtube3-cli) [Engine generation source](https://github.com/Byron/google-apis-rs/blob/main/src/generator/templates/cli/lib/engine.mako)

### Twitch 
