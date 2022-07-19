use std::{env, ops::Deref};

use eyre::{bail, Result};
use load_dotenv::load_dotenv;
use serde::{Deserialize, Serialize};
use url::Url;
use yewdux::prelude::{BasicStore, DispatchProps, Dispatcher};

use crate::utilities::{
    cookie::{get_cookie, set_cookie},
    create_uri,
    log::log_error,
    UriQueryParam,
};

load_dotenv!();

#[derive(Clone, Default)]
pub struct AuthStore {
    pub is_authenticated: bool,
    pub user: Option<User>,
    pub error: Option<String>,
}

pub fn create_login_uri(state: &str) -> String {
    format!("https://{}/authorize?response_type=token&client_id={}&connection={}&redirect_uri={}&scope=openid%20profile%20email&state={}", 
        env!("AUTH0_DOMAIN"),
        env!("AUTH0_CLIENT_ID"),
        env!("AUTH0_CONNECTION"),
        env!("AUTH0_REDIRECT_URI"),
        state
    )
}

pub fn get_auth_data_from_url() -> Result<Auth0UriResponse> {
    Auth0UriResponse::create()
}

pub fn compare_state_with_cookie(auth0_state: &str) -> bool {
    if let Some(state_cookie) = get_cookie("auth0_state") {
        gloo::console::log!(auth0_state, &state_cookie);
        auth0_state == state_cookie
    } else {
        false
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct User {}

impl User {
    pub async fn get_user(token: &str) -> Result<Self> {
        if token.is_empty() {
            bail!("Access Token missing or is invalid");
        }

        let url = format!("https://{}/userinfo", env!("AUTH0_DOMAIN"));
        let authorization_header = format!("Bearer {token}");
        let user = gloo::net::http::Request::get(&url)
            .header("Authorization", &authorization_header)
            .send()
            .await?
            .json::<Self>()
            .await?;
        Ok(user)
    }
}

#[derive(Default)]
pub struct Auth0UriResponse {
    pub state: Option<String>,
    pub token: Option<String>,
}

impl Auth0UriResponse {
    pub fn create() -> Result<Self> {
        let uri = match gloo::utils::window().location().href() {
            Ok(uri) => uri,
            Err(error) => bail!("Error getting browser URI: {:?}", error),
        };

        gloo::console::log!("uri:", &uri);

        let parsed_url = match Url::parse(&uri) {
            Ok(parsed_url) => parsed_url,
            Err(error) => bail!("Error parsing URI: {:?}", error),
        };

        let mut auth0_uri_response = Self::default();

        if let Some(fragment) = parsed_url.fragment() {
            for (key, value) in url::form_urlencoded::parse(fragment.as_bytes()) {
                match key.deref() {
                    "state" => {
                        auth0_uri_response.state = {
                            gloo::console::log!("extracting state from url", value.to_string());
                            Some(value.to_string())
                        }
                    }
                    "access_token" => auth0_uri_response.token = Some(value.to_string()),
                    _ => continue,
                }
            }
        }

        Ok(auth0_uri_response)
    }
}
