use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub alt: String,
    pub src: String,
    pub width: f32,
    pub height: f32,
}

#[styled_component(BBIcon)]
pub fn bb_icon(props: &Props) -> Html {
    html! {
        <img
            alt={props.alt.clone()}
            src={props.src.clone()}
            width={props.width.to_string()}
            height={props.height.to_string()}
            loading="eager"
        />
    }
}
