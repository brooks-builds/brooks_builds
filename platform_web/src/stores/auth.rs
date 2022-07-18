use load_dotenv::load_dotenv;

use crate::utilities::{cookie::get_cookie, create_uri, log::log_error, UriQueryParam};

load_dotenv!();

#[derive(Clone)]
pub struct AuthStore {
    pub domain: String,
    pub client_id: String,
    pub connection: String,
    pub redirect_uri: String,
    pub state: String,
}

impl AuthStore {
    pub fn create_login_uri(&self) -> String {
        let base_uri = format!("https://{}/authorize", self.domain);

        match create_uri(
            &base_uri,
            vec![
                UriQueryParam::new("response_type", "token"),
                UriQueryParam::new("client_id", &self.client_id),
                UriQueryParam::new("connection", &self.connection),
                UriQueryParam::new("redirect_uri", &self.redirect_uri),
                UriQueryParam::new("scope", "openid profile email"),
                UriQueryParam::new("state", &self.state),
                UriQueryParam::new("screen_hint", "signup"),
            ],
        ) {
            Ok(uri) => uri.to_string(),
            Err(error) => {
                log_error(&format!("Error creating login uri: {:?}", error));
                panic!();
            }
        }
    }
}

impl Default for AuthStore {
    fn default() -> Self {
        let domain = env!("AUTH0_DOMAIN");
        let client_id = env!("AUTH0_CLIENT_ID");
        let connection = env!("AUTH0_CONNECTION");
        let redirect_uri = env!("AUTH0_REDIRECT_URI");
        let state = get_cookie("auth0_state");

        Self {
            domain: domain.to_owned(),
            client_id: client_id.to_owned(),
            connection: connection.to_owned(),
            redirect_uri: redirect_uri.to_owned(),
            state: state.unwrap_or_default(),
        }
    }
}
