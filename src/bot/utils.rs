extern crate futures;
extern crate hyper;
extern crate core;
extern crate tokio_core;

use self::futures::future::Future;
use self::futures::Stream;
use self::hyper::Client;
use self::hyper::header::{Headers, ContentType};
use self::hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use self::hyper::Post;
use self::hyper::client::Request;

pub struct UrlRequest {}

impl UrlRequest {
    pub fn new() -> UrlRequest {
        UrlRequest {}
    }

    pub fn post(self,
                url: String,
                data: String,
                body: String)
                -> Box<Future<Item = String, Error = hyper::Error>> {
        // create request url
        let request_url = format!("{}{}{}", url, "?", data);
        let url = request_url.parse::<hyper::Uri>().unwrap();
        // headers
        let mut headers = Headers::new();
        headers.set(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])));

        let core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        let client = Client::new(&handle);
        let mut request = Request::new(Post, url);
        *(request.headers_mut()) = headers;
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
