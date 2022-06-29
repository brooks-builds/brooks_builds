use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub x: u8,
    pub y: u8,
    pub size: u8,
    pub color: String,
}

#[styled_component(BackgroundCircle)]
pub fn background_circle(props: &Props) -> Html {
    let style = use_style(create_css(props).as_str());

    html! {
        <div class={style}></div>
    }
}

fn create_css(props: &Props) -> String {
    let size = props.size;
    let top = props.y;
    let left = props.x;
    let color = &props.color;

    format!(
        "
        width: {size}rem;
        height: {size}rem;
        border-radius: 100%;
        background-color: {color};
        position: fixed;
        top: {top}dvh;
        left: {left}dvw;
        z-index: 0;
    "
    )
}
