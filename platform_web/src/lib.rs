mod components;
mod router;
mod utilities;

use components::molecules::{background::Background, navbar::TopNavbar};
use router::{switch, Route};
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
use yew_router::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let style = use_style(create_css());
    utilities::set_cookie("test", "test_cookie");

    html! {
        <BrowserRouter>
            <div class={style}>
                <TopNavbar />
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
