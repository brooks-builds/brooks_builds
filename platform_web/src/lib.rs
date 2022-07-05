mod components;

use components::molecules::{background::Background, navbar::TopNavbar};
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let style = use_style(create_css());

    html! {
        <>
            <TopNavbar />
            <Background />
        </>
    }
}

fn create_css() -> &'static str {
    r#"
        z-index: 100;
    "#
}
