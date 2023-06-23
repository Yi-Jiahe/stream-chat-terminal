# stream-chat-terminal

Display your stream chat in the terminal!

Polls YouTube chat list API to stream chat to your terminal.

## To-Do

### YouTube

[x] Implement OAuth2 to post chat messages

https://developers.google.com/identity/protocols/oauth2

https://crates.io/crates/oauth2

[ ] Switch to [google-youtube3](https://crates.io/crates/google-youtube3) crate for youtube functions.

I don't know why I didn't use it from the start

Seems like it doesn't support requests w/o OAuth, i.e. the user needs to sign in even to perform API requests that do not require an OAuth token.

### Twitch 
