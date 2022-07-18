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

use crate::stores::auth::{handle_redirect_callback, AuthStore};

#[styled_component(App)]
pub fn app() -> Html {
    let style = use_style(create_css());
    let store = use_store::<BasicStore<AuthStore>>();
    let store_loading = store.state().map(|store| store.loading).unwrap_or_default();

    {
        let dispatch = store.dispatch().clone();
        use_effect_once(move || {
            dispatch.reduce(|store| handle_redirect_callback(store));

            || {}
        });
    }

    html! {
        <BrowserRouter>
            <div class={style}>
                <TopNavbar />
                <Switch<Route> render={Switch::render(switch)} />
            </div>
            <Background />
            <pre>{store_loading}</pre>
        </BrowserRouter>
    }
}

fn create_css() -> &'static str {
    r#"
        z-index: 100;
        position: relative;
    "#
}
