use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub top: u8,
    pub left: u8,
    pub size: u8,
    pub color: String,
}

#[styled_component(BackgroundTriangle)]
pub fn background_triangle(props: &Props) -> Html {
    let css = use_style(create_css(props).as_str());

    html! {
        <div class={css}></div>
    }
}

fn create_css(props: &Props) -> String {
    let top = props.top;
    let left = props.left;
    let size = props.size;
    let bottom_side = ((size as f32 + size as f32) * 0.866) as u8;
    let color = &props.color;

    format!(
        "
        width: 0;
        height: 0;
        border-left: {size}rem solid transparent;
        border-right: {size}rem solid transparent;
        border-bottom: {bottom_side}rem solid {color};
        position: fixed;
        top: {top}dvh;
        left: {left}dvh;
        z-index: 0;
    "
    )
}
