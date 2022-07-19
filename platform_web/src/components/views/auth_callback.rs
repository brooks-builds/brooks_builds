use std::collections::HashMap;

use load_dotenv::load_dotenv;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::{history::History, hooks::use_history};
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::use_store;

use crate::{
    router::Route,
    stores::auth::{handle_redirect_callback, AuthStore},
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
        auth_store.dispatch().reduce(|store| {
            handle_redirect_callback(store);
        });
        || {}
    });
    // let history = use_history().unwrap();

    // let raw_uri = match gloo::utils::window().location().href() {
    //     Ok(uri) => uri,
    //     Err(error) => {
    //         log_error(&format!("Error parsing url in auth callback: {:?}", error));
    //         panic!();
    //     }
    // };
    // let parsed_url = match url::Url::parse(&raw_uri) {
    //     Ok(uri) => uri,
    //     Err(error) => {
    //         log_error(&format!("Error parsing auth callback uri: {:?}", error));
    //         panic!();
    //     }
    // };

    // if let Some(params) = parsed_url.fragment() {
    //     let hashed_params = parse_url_params(params);
    //     if compare_state_with_cookie(&hashed_params) {
    //         let auth_params = hashed_params.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //             let user_profile = get_user_profile(auth_params).await;
    //         });
    //     } else {
    //         log_error("Cannot trust login, states don't match");
    //         // set_cookie("auth0_state", "", "/", 0);
    //         // history.push(Route::Home);
    //     }
    // }

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

fn compare_state_with_cookie(params: &HashMap<String, String>) -> bool {
    let cookie_state = if let Some(cookie_state) = get_cookie("auth0_state") {
        cookie_state
    } else {
        log_error("could not find cookie auth0_state when trying to log in");
        return false;
    };
    let param_state = if let Some(state) = params.get("state") {
        state.to_owned()
    } else {
        log_error("could not find state in params when trying to log in");
        return false;
    };

    gloo::console::log!(cookie_state.clone(), param_state.clone());
    cookie_state == param_state
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
