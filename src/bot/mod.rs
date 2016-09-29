extern crate hyper;
extern crate multipart;

use self::hyper::client::Request;
use self::hyper::client::Response;
use self::hyper::method::Method;
use self::hyper::net::Streaming;

use self::multipart::client::Multipart;


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
        let mut request_endpoint = format!("{}{}", self.graph_url, "/me/messages");

        println!("{}", request_endpoint);

        let request = Request::new(Method::Post,
                                   request_endpoint.to_string().parse().expect("Failed")
                                   ).expect("Failed");
        let mut multipart = Multipart::from_request(request).expect("Failed");

        let body = format!("{}{}", "access_token=", self.access_token);
        let response = multipart.send().expect("Failed");

        println!("{}", response.status.to_string());

    }
}
