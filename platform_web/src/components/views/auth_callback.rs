use std::ops::Deref;

use yew::prelude::*;

use crate::utilities::{cookie::get_cookie, log::log_error};

#[function_component(AuthCallback)]
pub fn auth_callback() -> Html {
    let raw_uri = match gloo::utils::window().location().href() {
        Ok(uri) => uri,
        Err(error) => {
            log_error(&format!("Error parsing url in auth callback: {:?}", error));
            panic!();
        }
    };
    let parsed_url = match url::Url::parse(&raw_uri) {
        Ok(uri) => uri,
        Err(error) => {
            log_error(&format!("Error parsing auth callback uri: {:?}", error));
            panic!();
        }
    };

    if let Some(params) = parsed_url.fragment() {
        for (key, value) in url::form_urlencoded::parse(params.as_bytes()) {
            let state_from_cookie = get_cookie("state");
            gloo::console::log!(key.deref());
        }
    }

    html! {
        <h1>{"Auth Callback"}</h1>
    }
}
