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

    pub fn send_raw(self, payload: &str) {
        let request_endpoint = format!("{}{}", self.graph_url, "/me/messages");
        let url_request = utils::UrlRequest::new();

        let data = format!("{}{}", "access_token=", self.access_token).to_string();

        let body = "{'recipient': {'id': 1156130217782534}, 'message': {'text': 'test'}}".to_string();
        let response = url_request.post(request_endpoint, data, body);
    }
}
