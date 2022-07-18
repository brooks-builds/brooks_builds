use crate::{
    stores::auth::{self, AuthStore},
    utilities::{
        cookie::set_cookie, create_random_string, create_uri, log::log_error, UriQueryParam,
    },
};
use load_dotenv::load_dotenv;
use rand::distributions::{Alphanumeric, DistString};
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use yew::prelude::*;
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::use_store;

load_dotenv!();

#[function_component(TopNavbar)]
pub fn top_navbar() -> Html {
    let signup_onclick = {
        let auth_store = use_store::<BasicStore<AuthStore>>();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            auth_store
                .dispatch()
                .reduce(|store| store.state = create_random_string(43));
            let login_uri = auth_store
                .state()
                .clone()
                .map(|store| store.create_login_uri())
                .unwrap_or_default();
            let state = auth_store
                .state()
                .map(|store| store.state.clone())
                .unwrap_or_default();
            set_cookie("auth0_state", &state, "/", 60);
            if let Err(error) = gloo::utils::window().location().set_href(&login_uri) {
                log_error(&format!("Error navigating to Auth0 signup: {:?}", error));
            }
        })
    };

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
