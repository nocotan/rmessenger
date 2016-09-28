extern crate hyper;

use self::hyper::client::Response;

mod graph_api;
mod utils;


pub struct Bot {
    access_token: String,
    app_secret: String,
}

impl Bot {
    pub fn new(access_token: &str, app_secret: &str) -> Bot {
        Bot {
            access_token: access_token.to_string(),
            app_secret: app_secret.to_string(),
        }
    }

    pub fn send_raw(self, payload: &str) -> String {
        let root_api = graph_api::FacebookGraphApi::new(self.access_token, self.app_secret);
        let api = graph_api::FacebookGraphApi{..root_api};

        let request_endpoint = api.auth_url();

        let request = utils::UrlRequest::new();

        let access_token = api.get_access_token();
        let body = format!("{}{}", "access_token=", access_token);
        let response = request.post(request_endpoint, body);

        response.status.to_string()
    }
}
