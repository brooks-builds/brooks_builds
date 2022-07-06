use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub size: u8,
    pub x: u8,
    pub y: u8,
    pub color: String,
}

#[styled_component(BackgroundSquare)]
pub fn background_square(props: &Props) -> Html {
    let css = use_style(create_css(props).as_str());

    html! {
        <div class={css}></div>
    }
}

fn create_css(props: &Props) -> String {
    let color = &props.color;
    let x = props.x;
    let y = props.y;
    let size = props.size;

    format!(
        r#"
        position: fixed;
        width: {size}rem;
        height: {size}rem;
        top: {y}%;
        left: {x}%;
        background-color: {color};
        z-index: 0;
    "#
    )
}
