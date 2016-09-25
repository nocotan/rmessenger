/// ##struct of Facebook Graph API
/// ###require:
/// - access_token
/// ###optional:
/// - app_secret
pub struct FacebookGraphApi {
    api_version: i32,
    app_secret: String,
    graph_url: String,
    access_token: String,
}

impl FacebookGraphApi {
    fn new(access_token: &str, app_secret: Option<i32>) -> FacebookGraphApi {
        FacebookGraphApi {
            api_version: 2.6,
            app_secret: if app_secret == None { "" } else { app_secret },
            graph_url: "https://graph.facebook.com/v2.6",
            access_token: access_token,
        }
    }
}
