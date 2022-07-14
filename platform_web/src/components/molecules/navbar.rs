use crate::utilities::{
    cookie::set_cookie, create_random_string, create_uri, log::log_error, UriQueryParam,
};
use load_dotenv::load_dotenv;
use rand::distributions::{Alphanumeric, DistString};
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use yew::prelude::*;

load_dotenv!();

#[function_component(TopNavbar)]
pub fn top_navbar() -> Html {
    let signup_onclick = Callback::from(|event: MouseEvent| {
        event.prevent_default();
        let state = create_random_string(43);
        let login_uri = create_login_uri(&state);
        set_cookie("auth0_state", &state);
        if let Err(error) = gloo::utils::window().location().set_href(&login_uri) {
            log_error(&format!("Error navigating to Auth0 signup: {:?}", error));
        }
    });

    html! {
        <nav class="navbar navbar-expand-lg">
            <div class="container-fluid">
                <div>
                    <img class="navbar-brand" src="/static/brooks.png" alt="Brooks' logo" data-test="nav-logo" />
                    <span class="navbar-text" data-test="nav-title">{"Brooks Builds"}</span>
                </div>
                <div>
                    <a href="#" data-test="auth-sign-up" onclick={signup_onclick}>{"Sign Up"}</a>
                </div>
            </div>
        </nav>
    }
}

fn create_login_uri(state: &str) -> String {
    let domain = env!("AUTH0_DOMAIN");
    let client_id = env!("AUTH0_CLIENT_ID");
    let connection = env!("AUTH0_CONNECTION");
    let redirect_uri = env!("AUTH0_REDIRECT_URI");
    let base_uri = format!("https://{domain}/authorize");
    let state = create_random_string(43);
    match create_uri(
        &base_uri,
        vec![
            UriQueryParam::new("response_type", "token"),
            UriQueryParam::new("client_id", client_id),
            UriQueryParam::new("connection", connection),
            UriQueryParam::new("redirect_uri", redirect_uri),
            UriQueryParam::new("scope", "openid profile email"),
            UriQueryParam::new("state", &state),
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
