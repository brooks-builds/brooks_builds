use rand::Rng;
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[styled_component(BackgroundCircle)]
pub fn background_circle() -> Html {
    let style = use_style(create_css().as_str());

    html! {
        <div class={style}></div>
    }
}

fn create_css() -> String {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(3..25);
    let top = rng.gen_range(0..95);
    let left = rng.gen_range(0..95);
    format!(
        "
        width: {size}rem;
        height: {size}rem;
        border-radius: 100%;
        background-color: rgba(255, 0, 0, 0.1);
        position: fixed;
        top: {top}dvh;
        left: {left}dvw;
        z-index: 0;
    "
    )
}
