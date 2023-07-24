# stream-chat-terminal

Display your stream chat in the terminal!

Polls YouTube chat list API to stream chat to your terminal.

## Setup

A client secret for desktop application has been provided for the application. However, at ~5 units per call of the live chat messages list endpoint and a max quota of 10,000, the quota is easily exceeded within 2 hours at ~0.3 calls per second.

In case you need to provide your own client secret to bypass this shared quota, you can place the client secret downloaded as json from the gcp console in the application config directory as `youtube3-secret.json`. The application config directory can be quried with the `--config-path` flag.
