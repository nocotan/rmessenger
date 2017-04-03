extern crate futures;
extern crate hyper;
extern crate url;

mod utils;
use self::futures::future::Future;
use self::url::form_urlencoded;

#[derive(Clone)]
pub struct Bot {
    access_token: String,
    app_secret: String,
    webhook_verify_token: String,
    graph_url: String,
}

impl Bot {
    pub fn new(access_token: &str, app_secret: &str, webhook_verify_token: &str) -> Bot {
        Bot {
            access_token: access_token.to_string(),
            app_secret: app_secret.to_string(),
            webhook_verify_token: webhook_verify_token.to_string(),
            graph_url: "https://graph.facebook.com/v2.7".to_string(),
        }
    }

    /// Verify the Get query (after the ?) of a webhook verification request
    /// (see https://developers.facebook.com/docs/graph-api/webhooks#setup)
    /// and return either Some(hub.challenge) for you to put in the body of your
    /// response, or None.
    pub fn verify_webhook_query(&self, query: &str) -> Option<String> {
        let mut maybe_hub_challenge = None;
        let mut webhook_verify_token = false;

        for (key, value) in form_urlencoded::parse(query.as_bytes()) {
            if key == "hub.challenge" {
                println!("hub.challenge received");
                maybe_hub_challenge = Some(value.into_owned());
            } else if key == "hub.verify_token" && value == self.webhook_verify_token {
                println!("hub.verify_token passed");
                webhook_verify_token = true;
            }
        }
        if webhook_verify_token {
            return maybe_hub_challenge;
        } else {
            return None;
        }
    }

    /// send text messages to the specified recipient.
    pub fn send_text_message(&self,
                             recipient_id: &str,
                             message: &str)
                             -> Box<Future<Item = String, Error = hyper::Error>> {
        let payload = format!("
                              {{
                                 'recipient': {{'id': {} }},
                                 'message': {{'text': '{}'}}
                              }}",
                              recipient_id,
                              message);

        self.send_raw(payload.to_string())
    }

    /// send generic message to the specified recipient.
    pub fn send_generic_message(&self,
                                recipient_id: &str,
                                elements: &str)
                                -> Box<Future<Item = String, Error = hyper::Error>> {
        let payload = format!("
                              {{
                                'recipient': {{'id': {} }},
                                'message': {{
                                  'attachment': {{
                                    'type': 'template',
                                    'payload': {{
                                      'template_type': 'generic',
                                      'elements': {}
                                    }}
                                  }}
                                }}
                              }}",
                              recipient_id,
                              elements);

        self.send_raw(payload.to_string())
    }

    /// send button message to the specified recipient.
    pub fn send_button_message(&self,
                               recipient_id: &str,
                               text: &str,
                               buttons: &str)
                               -> Box<Future<Item = String, Error = hyper::Error>> {
        let payload = format!("
                              {{
                                'recipient': {{'id': {} }},
                                'message': {{
                                  'attachment': {{
                                    'type': 'template',
                                    'payload': {{
                                      'template_type': 'button',
                                      'text': {},
                                      'buttons': {}
                                    }}
                                  }}
                                }}
                              }}",
                              recipient_id,
                              text,
                              buttons);

        self.send_raw(payload.to_string())
    }

    /// send file url to the specified recipient.
    pub fn send_file_url(&self,
                         recipient_id: &str,
                         file_url: &str)
                         -> Box<Future<Item = String, Error = hyper::Error>> {
        let payload = format!("
                              {{
                                'recipient': {{'id': {} }},
                                'message': {{
                                  'attachment': {{
                                    'type': 'file',
                                    'payload': {{
                                      'url': {}
                                    }}
                                  }}
                                }}
                              }}",
                              recipient_id,
                              file_url);

        self.send_raw(payload.to_string())
    }

    /// send audio url to the specified recipient.
    pub fn send_audio_url(&self,
                          recipient_id: &str,
                          audio_url: &str)
                          -> Box<Future<Item = String, Error = hyper::Error>> {
        let payload = format!("
                              {{
                                'recipient': {{'id': {} }},
                                'message': {{
                                  'attachment': {{
                                    'type': 'audio',
                                    'payload': {{
                                      'url': {}
                                    }}
                                  }}
                                }}
                              }}",
                              recipient_id,
                              audio_url);

        self.send_raw(payload.to_string())
    }

    /// send payload.
    fn send_raw(&self, payload: String) -> Box<Future<Item = String, Error = hyper::Error>> {
        let request_endpoint = format!("{}{}", self.graph_url, "/me/messages");
        let url_request = utils::UrlRequest::new();

        let data = format!("{}{}", "access_token=", self.access_token).to_string();

        url_request.post(request_endpoint, data, payload)
    }
}
