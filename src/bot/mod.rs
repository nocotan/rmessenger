mod utils;

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

    /// send text messages to the specified recipient.
    pub fn send_text_message(self, recipient_id: &str, message: &str) -> String {
        let payload = format!(
            "{{
                'recipient': {{'id': {} }},
                'message': {{'text': '{}'}}
             }}",
             recipient_id, message);

        self.send_raw(payload.to_string())
    }

    /// send generic message to the specified recipient.
    pub fn send_generic_message(self, recipient_id: &str, elements: &str) -> String {
        let payload = format!(
            "{{
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
            recipient_id, elements);

        self.send_raw(payload.to_string())
    }

    /// send button message to the specified recipient.
    pub fn send_button_message(self, recipient_id: &str, text: &str, buttons: &str) -> String {
        let payload = format!(
            "{{
                'recipient': {{'id': {} }},
                'message': {{
                  'attachment': {{
                    'type': 'template',
                    'payload': {{
                      'template_type': 'button',
                      'text': {},
                      'buttons': buttons
                    }}
                  }}
                }}
             }}",
             recipient_id, text, buttons);

        self.send_raw(payload.to_string())
    }

    fn send_raw(self, payload: String) -> String {
        let request_endpoint = format!("{}{}", self.graph_url, "/me/messages");
        let url_request = utils::UrlRequest::new();

        let data = format!("{}{}", "access_token=", self.access_token).to_string();


        url_request.post(request_endpoint, data, payload)
    }
}
