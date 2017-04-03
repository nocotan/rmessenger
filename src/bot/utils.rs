extern crate hyper;
extern crate core;

use std::io::Read;

use self::hyper::Client;
use self::hyper::header::{Headers, ContentType};
use self::hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};


pub struct UrlRequest {}

impl UrlRequest {
    pub fn new() -> UrlRequest {
        UrlRequest {}
    }

    pub fn post(self, url: String, data: String, body: String) -> String {
        // create request url
        let request_url = format!("{}{}{}", url, "?", data);

        // headers
        let mut headers = Headers::new();
        headers.set(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])));

        // client
        let client = Client::new();
        let res = client
            .post(&request_url.to_owned())
            .headers(headers)
            .body(&body.to_owned())
            .send();

        let mut response_body = String::new();
        res.unwrap().read_to_string(&mut response_body).unwrap();
        println!("{}", response_body);

        response_body
    }
}
