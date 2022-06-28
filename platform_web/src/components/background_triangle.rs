use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {}

#[styled_component(BackgroundTriangle)]
pub fn background_triangle(props: &Props) -> Html {
    let css = use_style(create_css(props).as_str());

    html! {
        <div class={css}></div>
    }
}

fn create_css(props: &Props) -> String {
    format!("")
}
