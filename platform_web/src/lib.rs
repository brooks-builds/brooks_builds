mod components;
mod router;

use components::molecules::{background::Background, navbar::TopNavbar};
use router::{switch, Route};
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
use yew_router::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let style = use_style(create_css());

    html! {
        <BrowserRouter>
            <TopNavbar />
            <Switch<Route> render={Switch::render(switch)} />
            <Background />
        </BrowserRouter>
    }
}

fn create_css() -> &'static str {
    r#"
        z-index: 100;
    "#
}
