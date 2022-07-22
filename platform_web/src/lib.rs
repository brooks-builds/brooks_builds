mod components;
mod router;
mod stores;
mod utilities;

use components::molecules::{background::Background, navbar::TopNavbar};
use router::{switch, Route};
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::*;
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::use_store;

use crate::{
    stores::auth::{get_user_profile, AuthStore},
    utilities::cookie::get_cookie,
};

#[styled_component(App)]
pub fn app() -> Html {
    let style = use_style(create_css());
    let auth_error = use_store::<BasicStore<AuthStore>>()
        .state()
        .map(|store| store.error.clone().unwrap_or_default())
        .unwrap_or_default();

    {
        let auth_store = use_store::<BasicStore<AuthStore>>();
        use_effect_once(move || {
            if let Some(token) = get_cookie("auth0_token") {
                let auth_store_dispatch = auth_store.dispatch().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match get_user_profile(&token).await {
                        Ok(profile) => {
                            gloo::console::log!("got profile", &profile.nickname);
                            auth_store_dispatch.reduce(|store| {
                                store.user = Some(profile);
                                store.is_authenticated = true
                            })
                        }
                        Err(_) => auth_store_dispatch.reduce(|store| {
                            store.is_authenticated = false;
                            store.token = None;
                            store.user = None;
                        }),
                    }
                })
            } else {
                auth_store.dispatch().reduce(|store| {
                    store.is_authenticated = false;
                    store.token = None;
                    store.user = None;
                });
            }

            || {}
        });
    }
    html! {
        <BrowserRouter>
            <div class={style}>
                <TopNavbar />
                <div>{auth_error}</div>
                <Switch<Route> render={Switch::render(switch)} />
            </div>
            <Background />
        </BrowserRouter>
    }
}

fn create_css() -> &'static str {
    r#"
        z-index: 100;
        position: relative;
    "#
}
