extern crate hyper;

use self::hyper::client::Response;

mod graph_api;
mod utils;


pub struct Bot {
    api: graph_api::FacebookGraphApi,
}

impl Bot {
    pub fn new(access_token: &str, app_secret: &str) -> Bot {
        Bot {
            api: graph_api::FacebookGraphApi::new(access_token, app_secret)
        }
    }

    pub fn send_raw(self, payload: &str) -> String {
        let request_endpoint = self.api.get_graph_url()
            + "/me/messages";

        let request = utils::UrlRequest::new();
        let response = request.post(request_endpoint, "0000");

        response.status.to_string()
    }
}
