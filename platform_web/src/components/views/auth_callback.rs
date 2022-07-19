use std::collections::HashMap;

use load_dotenv::load_dotenv;
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, use_effect_once};
use yew_router::{history::History, hooks::use_history};
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::use_store;

use crate::{
    router::Route,
    stores::auth::{self, compare_state_with_cookie, get_auth_data_from_url, AuthStore},
    utilities::{
        cookie::{get_cookie, set_cookie},
        log::log_error,
    },
};

load_dotenv!();

#[function_component(AuthCallback)]
pub fn auth_callback() -> Html {
    let auth_store = use_store::<BasicStore<AuthStore>>();

    use_effect_once(move || {
        match get_auth_data_from_url() {
            Ok(auth_uri_response) => {
                let uri_state = auth_uri_response.state.unwrap_or_default();
                if !compare_state_with_cookie(&uri_state) {
                    auth_store.dispatch().reduce(|store| {
                        store.error = Some("Invalid State, please try logging in again".to_owned());
                    });
                }
            }
            Err(error) => {
                auth_store
                    .dispatch()
                    .reduce(move |store| store.error = Some(format!("{:?}", error)));
            }
        }

        // let history = use_history().unwrap().push(Route::Home);
        || {}
    });

    html! {
        <h1>{"Auth Callback"}</h1>
    }
}

fn parse_url_params(params: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for (key, value) in url::form_urlencoded::parse(params.as_bytes()) {
        result.insert(key.to_string(), value.to_string());
    }

    result
}

async fn get_user_profile(auth_params: HashMap<String, String>) -> () {
    let access_token = if let Some(token) = auth_params.get("access_token") {
        token.to_owned()
    } else {
        log_error("cannot get user profile because the token doesn't exist");
        panic!();
    };

    let auth0_domain = env!("AUTH0_DOMAIN");
    let url = format!("{auth0_domain}/userinfo");
    let authorization_header = format!("Bearer {access_token}");
    let result = gloo::net::http::Request::get(&url)
        .header("Authorization", &authorization_header)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    gloo::console::log!(result);
}
