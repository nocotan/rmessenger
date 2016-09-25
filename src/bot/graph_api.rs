/// ##struct of Facebook Graph API
/// ###require:
/// - access_token
/// ###optional:
/// - app_secret
pub struct FacebookGraphApi {
    api_version: f32,
    app_secret: String,
    graph_url: String,
    access_token: String,
}

impl FacebookGraphApi {
    pub fn new(access_token: &str, app_secret: &str) -> FacebookGraphApi {
        FacebookGraphApi {
            api_version: 2.6,
            app_secret: app_secret.to_string(),
            graph_url: "https://graph.facebook.com/v2.6".to_string(),
            access_token: access_token.to_string(),
        }
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
