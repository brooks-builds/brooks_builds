use crate::background_circle::BackgroundCircle;
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[styled_component(Background)]
pub fn background() -> Html {
    let style = use_style(create_css());
    let mut background_elements = vec![];
    for _ in 0..5 {
        background_elements.push(html! { <BackgroundCircle /> });
    }

    html! {
        <div class={style}>
            {background_elements}
        </div>
    }
}

fn create_css() -> &'static str {
    r#"
    "#
}
