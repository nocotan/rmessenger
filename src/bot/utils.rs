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
        let mut request_url = format!("{}{}{}", url, "?", data);
        println!("{}", request_url);

        let request = Request::new(
                Method::Post,
                request_url.to_string().parse().expect("Failed to parse url.")
            ).expect("Failed to create request.");

        let mut multipart = Multipart::from_request(request).expect("Failed to create multipart.");
        multipart.send().expect("Failed to send request")
    }
}
