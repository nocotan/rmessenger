extern crate hyper;
extern crate core;

use std::io::Read;

use self::hyper::Client;
use self::hyper::header::Connection;
use self::hyper::header::Basic;
use self::hyper::header::{Headers, ContentType};
use self::hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use self::core::str::FromStr;


pub struct UrlRequest { }

impl UrlRequest {
    pub fn new() -> UrlRequest {
        UrlRequest { }
    }

    pub fn post(self, url: String, data: String, body: String) -> String {
        // create request url
        let mut request_url = format!("{}{}{}", url, "?", data);

        // headers
        let mut headers = Headers::new();
        headers.set(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])
                                ));

        let mut client = Client::new();
        let mut res = client.post("https://graph.facebook.com/v2.6/me/messages?access_token=EAACXVkA4ZCL8BAK6UGzJxeqt4M8HxUsWyH4yZCZA68iRsSNZAyP9ZBNb6lZC7F5p5Wrnd9ZCZCZA7hL9fcqcbYsZBmO6aOTuFBX08oiFtj1vU9XLsLDEzZC3THwwHoj6sMYJVf7wvdGF5oCWEaOAl2kl934EyyOeT2P9f5oXHffM6ovRAZDZD")
                            .headers(headers)
                            .body(&body.to_owned())
                            .send();

        let mut response_body = String::new();
        res.unwrap().read_to_string(&mut response_body).unwrap();

        response_body
    }
}
