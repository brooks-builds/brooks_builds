use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

use super::{get_html_document, log::log_error};

pub fn set_cookie(key: &str, value: &str, path: &str, ttl: u32) {
    let document = get_html_document();
    let cookie = format!("{key}={value}; SameSite=Strict; Secure; path={path}; max-age={ttl}");
    match document.set_cookie(&cookie) {
        Ok(_) => {}
        Err(error) => gloo::console::error!("Error setting cookie", error),
    };
}

pub fn get_cookie(key: &str) -> Option<String> {
    let document = get_html_document();
    match document.cookie() {
        Ok(cookies) => cookies
            .split("; ")
            .find(|key_pair| key_pair.starts_with(key))
            .map(|key_value| &key_value[key.len() + 1..])
            .map(|value| value.to_owned()),
        Err(error) => {
            log_error(&format!("Error getting cookies: {:?}", error));
            panic!();
        }
    }
}
