extern crate hyper;
extern crate multipart;

use self::hyper::client::Request;
use self::hyper::client::Response;
use self::hyper::method::Method;
use self::hyper::net::Streaming;

use self::multipart::client::Multipart;


pub struct UrlRequest { }

impl UrlRequest {
    pub fn new() -> UrlRequest {
        UrlRequest { }
    }

    pub fn post(self, url: String, data: String) -> Response {
        let req = Request::new(Method::Post, url.parse().expect("Failed"))
            .expect("Failed to create request");

        let mut multipart = Multipart::from_request(req)
            .expect("Failed to create Multipart");

        multipart.write_text("text", data);

        multipart.send().expect("Failed to send request")
    }
}
