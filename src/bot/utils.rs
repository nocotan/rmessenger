extern crate futures;
extern crate hyper;
extern crate core;
extern crate tokio_core;
extern crate hyper_tls;

use self::futures::future::Future;
use self::futures::Stream;
use self::hyper::header::ContentType;
use self::hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use self::hyper::Post;
use self::hyper::client::Request;

pub struct UrlRequest {}

impl UrlRequest {
    pub fn new() -> UrlRequest {
        UrlRequest {}
    }

    pub fn post(self,
                client: hyper::Client<hyper_tls::HttpsConnector>,
                url: String,
                data: String,
                body: String)
                -> Box<Future<Item = String, Error = hyper::Error>> {

        let request_url = format!("{}{}{}", url, "?", data).parse().unwrap();
        let mut request = Request::new(Post, request_url);
        request
            .headers_mut()
            .set(ContentType(Mime(TopLevel::Application,
                                  SubLevel::Json,
                                  vec![(Attr::Charset, Value::Utf8)])));
        request.set_body(body.to_owned());

        let fut = client
            .request(request)
            .and_then(|res| {
                          res.body()
                              .fold(Vec::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, hyper::Error>(acc)
                })
                      })
            .map(|vec| String::from_utf8(vec).unwrap());
        Box::new(fut)
    }
}
