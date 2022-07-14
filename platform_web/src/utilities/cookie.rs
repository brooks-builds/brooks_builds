use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

use super::{get_html_document, log::log_error};

pub fn set_cookie(key: &str, value: &str) {
    let document = get_html_document();
    let cookie = format!("{key}={value}; SameSite=Strict; Secure;");
    match document.set_cookie(&cookie) {
        Ok(_) => {}
        Err(error) => gloo::console::error!("Error setting cookie", error),
    };
}

pub fn get_cookie(key: &str) -> Option<String> {
    let document = get_html_document();
    match document.cookie() {
        Ok(cookies) => {
            let parsed_cookies = parse_cookie(&cookies);
        }
        Err(error) => {
            log_error(&format!("Error getting cookies: {:?}", error));
            panic!();
        }
    }

    None
}

fn parse_cookie(cookies: &str) -> () {
    cookie::
}
