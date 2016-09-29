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

```rust
extern crate rmessenger;
use rmessenger::bot;

fn main() {
    let bot = bot::Bot::new("<YOUR ACCESS TOKEN>", "<YOUR APP SECRET>");
    bot.send_text_message("<recipient_id>", "<message>");
}
````

####Send generic message

```rust
extern crate rmessenger;
use rmessenger::bot;

fn main() {
    let bot = bot::Bot::new("<YOUR ACCESS TOKEN>", "<YOUR APP SECRET>");
    bot.send_generic_message("<recipient_id>>",
                             "[{'title': 'example',
                                'image_url': 'https://petersfancybrownhats.com/company_image.png'
                                }]");
}
````

> elements param is &str

###TODO
- send image
- send file
