# rmessenger
[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)  

##A Rust Wrapper for the FaceBook Messenger Bot API
[Facebook's Messenger Platform](https://developers.facebook.com/docs/messenger-platform)

###About
you can:
- send message

###Installation

###Usage

####Send text message
https://developers.facebook.com/docs/messenger-platform/send-api-reference/text-message

```rust::send_text_message.rs
extern crate rmessenger;
use rmessenger::bot;

fn main() {
    let bot = bot::Bot::new("<YOUR ACCESS TOKEN>", "<YOUR APP SECRET>");
    bot.send_text_message("<recipient_id>", "<message>");
}
````
