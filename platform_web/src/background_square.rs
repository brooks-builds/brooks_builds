use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub size: u8,
    pub x: u8,
    pub y: u8,
}

#[styled_component(BackgroundSquare)]
pub fn background_square(props: &Props) -> Html {
    let css = use_style(create_css(props.x, props.y, props.size).as_str());

    html! {
        <div class={css}></div>
    }
}

fn create_css(x: u8, y: u8, size: u8) -> String {
    gloo::console::log!(x, y, size);
    format!(
        r#"
        position: fixed;
        width: {size}rem;
        height: {size}rem;
        top: {y}%;
        left: {x}%;
        background-color: rgba(255, 0, 0, 0.1);
    "#
    )
}
