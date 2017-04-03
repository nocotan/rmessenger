extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate pretty_env_logger;
extern crate rmessenger;

use std::env;


fn main() {
    pretty_env_logger::init().unwrap();

    let access_token = env::var("ACCESS_TOKEN").unwrap_or(String::new());
    let app_secret = env::var("APP_SECRET").unwrap_or(String::new());
    let webhook_verify_token = env::var("WEBHOOK_VERIFY_TOKEN").unwrap_or(String::new());

    let recipient = match env::args().nth(1) {
        Some(recipient) => recipient,
        None => {
            println!("Usage: client <recipient> <text>");
            return;
        }
    };
    let text = match env::args().nth(2) {
        Some(text) => text,
        None => {
            println!("Usage: client <recipient> <text>");
            return;
        }
    };

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle))
        .build(&handle);


    let bot = rmessenger::bot::Bot::new(client.clone(),
                                        &access_token,
                                        &app_secret,
                                        &webhook_verify_token);

    let message_promise = bot.send_text_message(&recipient, &text);

    let response = core.run(message_promise);
    match response {
        Ok(res) => println!("Got response from facebook: {}", res),
        Err(err) => println!("Got error: {:?}", err),
    };
}
