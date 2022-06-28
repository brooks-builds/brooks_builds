mod background;
mod background_circle;
mod background_square;
mod components;

use background::Background;
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let style = use_style(create_css());

    html! {
        <>
        <div class={style}>
            <h1>{"I am an H1"}</h1>
            <p>{"I am a paragraph"}</p>
        </div>
        <Background />
        </>
    }
}

fn create_css() -> &'static str {
    r#"
        z-index: 100;
    "#
}
