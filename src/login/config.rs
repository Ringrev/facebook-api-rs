use serde::{Deserialize, Serialize};
/// A struct which describes the parameters used to construction of Facebook
/// login
#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    /// The Facebook url preamble for the oath dialog.
    ///
    /// This parameters is set to default value to
    /// https://www.facebook.com/v13.0/dialog/oauth?
    pub facebook_oath_url: String,

    /// The ID of your app, found in your app's dashboard.
    pub client_id: String,

    /// The URL that you want to redirect the person logging in back to.
    pub redirect_uri: String,
}

impl Config {
    pub fn new(client_id: String, redirect_uri: String) -> Self {
        Config {
            facebook_oath_url: "https://www.facebook.com/v13.0/dialog/oauth?".to_owned(),
            client_id,
            redirect_uri,
        }
    }

    pub fn facebook_oath_url(&self) -> &str {
        &self.facebook_oath_url
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }
}
