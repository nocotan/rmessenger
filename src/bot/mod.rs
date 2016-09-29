mod utils;


pub struct Bot {
    access_token: String,
    app_secret: String,
    graph_url: String,
}

impl Bot {
    pub fn new(access_token: &str, app_secret: &str) -> Bot {
        Bot {
            access_token: access_token.to_string(),
            app_secret: app_secret.to_string(),
            graph_url: "https://graph.facebook.com/v2.6".to_string(),
        }
    }

    pub fn send_text_message(self, recipient_id: &str, message: &str) -> String {
        let payload = format!("{}{}{}{}{}",
                              "{'recipient':
                                 {'id':", recipient_id,"},
                              'message':
                                 {'text': '", message, "'}
                               }"
                             );
        self.send_raw(payload.to_string())
    }

    fn send_raw(self, payload: String) -> String {
        let request_endpoint = format!("{}{}", self.graph_url, "/me/messages");
        let url_request = utils::UrlRequest::new();

        let data = format!("{}{}", "access_token=", self.access_token).to_string();


        url_request.post(request_endpoint, data, payload)
    }
}
