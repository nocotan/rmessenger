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

    pub fn post(self, url: String, body: String) {
        let mut request_url = format!("{}{}{}", url, "?", body);
        println!("{}", request_url);

        let mut client = Client::new();
        let mut headers = Headers::new();
        headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
        let mut res = client.post("https://graph.facebook.com/v2.6/me/messages?access_token=EAACXVkA4ZCL8BAK6UGzJxeqt4M8HxUsWyH4yZCZA68iRsSNZAyP9ZBNb6lZC7F5p5Wrnd9ZCZCZA7hL9fcqcbYsZBmO6aOTuFBX08oiFtj1vU9XLsLDEzZC3THwwHoj6sMYJVf7wvdGF5oCWEaOAl2kl934EyyOeT2P9f5oXHffM6ovRAZDZD").headers(headers).body("{'recipient': {'id': 1156130217782534}, 'message': {'text': 'test'}}").send();
        let mut b = String::new();
        res.unwrap().read_to_string(&mut b).unwrap();
        println!("{}", b);
        //let request = Request::new(
        //        Method::Post,
        //        request_url.to_string().parse().expect("Failed to parse url.")
        //    ).expect("Failed to create request.");

        //let mut multipart = Multipart::from_request(request).expect("Failed to create multipart.");
        //multipart.write_text("data", data.to_string());
        //multipart.send().expect("Failed to send request")
    }
}
