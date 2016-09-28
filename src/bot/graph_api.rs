/// ##struct of Facebook Graph API
/// ###require:
/// - access_token
/// ###optional:
/// - app_secret
pub struct FacebookGraphApi {
    pub api_version: f32,
    pub app_secret: String,
    pub graph_url: String,
    pub access_token: String,
}

impl FacebookGraphApi {
    pub fn new(access_token: String, app_secret: String) -> FacebookGraphApi {
        FacebookGraphApi {
            api_version: 2.6,
            app_secret: app_secret,
            graph_url: "https://graph.facebook.com/v2.6".to_string(),
            access_token: access_token,
        }
    }

    pub fn auth_url(self) -> String {
        format!("{}{}", self.get_graph_url(), "/me/messages")
    }

    pub fn get_app_secret(self) -> String {
        self.app_secret
    }

    pub fn get_graph_url(self) -> String {
        self.graph_url
    }

    pub fn get_access_token(self) -> String {
        self.access_token
    }

}
