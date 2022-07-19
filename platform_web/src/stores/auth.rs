use std::{env, ops::Deref};

use eyre::{bail, Result};
use load_dotenv::load_dotenv;
use url::Url;
use yewdux::prelude::{BasicStore, DispatchProps, Dispatcher};

use crate::utilities::{cookie::get_cookie, create_uri, log::log_error, UriQueryParam};

load_dotenv!();

#[derive(Clone, Default)]
pub struct AuthStore {
    pub loading: bool,
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

pub fn handle_redirect_callback(store: &mut AuthStore) {
    store.loading = true;
    match Auth0UriResponse::create() {
        Ok(auth0_uri_response) => {
            if compare_state_with_cookie(&auth0_uri_response.state.unwrap_or_default()) {
                gloo::console::log!("state matches!");
            } else {
                store.error = Some("Cannot log you in, state doesn't match".to_owned());
            }
        }
        Err(error) => {
            store.error = Some(format!("{:?}", error));
        }
    }
}

fn compare_state_with_cookie(auth0_state: &str) -> bool {
    if let Some(state_cookie) = get_cookie("auth0_state") {
        gloo::console::log!(&state_cookie, auth0_state);
        auth0_state == state_cookie
    } else {
        false
    }
}

#[derive(Default, Clone)]
pub struct User {}

#[derive(Default)]
struct Auth0UriResponse {
    pub state: Option<String>,
}

impl Auth0UriResponse {
    pub fn create() -> Result<Self> {
        let uri = match gloo::utils::window().location().href() {
            Ok(uri) => uri,
            Err(error) => bail!("Error getting browser URI: {:?}", error),
        };

        let parsed_url = match Url::parse(&uri) {
            Ok(parsed_url) => parsed_url,
            Err(error) => bail!("Error parsing URI: {:?}", error),
        };

        let mut auth0_uri_response = Self::default();

        if let Some(fragment) = parsed_url.fragment() {
            for (key, value) in url::form_urlencoded::parse(fragment.as_bytes()) {
                match key.deref() {
                    "state" => auth0_uri_response.state = Some(value.to_string()),
                    _ => continue,
                }
            }
        }

        Ok(auth0_uri_response)
    }
}
