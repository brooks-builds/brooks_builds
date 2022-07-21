use crate::router::Route;
use crate::stores::auth::{create_login_uri, create_logout_url};
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
use yew_router::prelude::*;
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::use_store;

load_dotenv!();

#[function_component(TopNavbar)]
pub fn top_navbar() -> Html {
    let auth_store = use_store::<BasicStore<AuthStore>>();
    let signup_onclick = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let state = create_random_string(43);
            let login_uri = create_login_uri(&state);
            set_cookie("auth0_state", &state, "/auth/callback", 60);
            gloo::console::log!("login uri:", &login_uri);
            if let Err(error) = gloo::utils::window().location().set_href(&login_uri) {
                log_error(&format!("Error navigating to Auth0 signup: {:?}", error));
            }
        })
    };

    let (is_authenticated, user) = auth_store
        .state()
        .map(|store| (store.is_authenticated, store.user.clone()))
        .unwrap_or_default();
    let nickname = user.map(|user| user.nickname).unwrap_or_default();
    let logout_url = create_logout_url();

    html! {
        <nav class="navbar navbar-expand-lg">
            <div class="container-fluid">
                <Link<Route> to={Route::Home}>
                    <img class="navbar-brand" src="/static/brooks.png" alt="Brooks' logo" data-test="nav-logo" />
                </Link<Route>>
                <div class="navbar-text" data-test="nav-title">{"Brooks Builds"}</div>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavAltMarkup" aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarNavAltMarkup">
                    <div class="me-auto"></div>
                    if is_authenticated {
                        <ul class="navbar-nav">
                            <li class="nav-item navbar-text">
                               {format!("Welcome, {}", nickname)}
                            </li>
                            <li class="nav-item">
                                <a href={logout_url} class="nav-link">{"Log Out"}</a>
                            </li>
                        </ul>
                    } else {
                        <a href="#" class="nav-link" data-test="auth-sign-up" onclick={signup_onclick}>{"Sign Up"}</a>
                    }
                </div>


                // <span>{"Welcome Username"}</span>
            </div>
        </nav>
    }
}
