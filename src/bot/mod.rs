mod graph_api;


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

        let response= "";

        response.to_string()
    }
}
