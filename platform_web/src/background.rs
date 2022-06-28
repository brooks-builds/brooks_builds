use crate::background_circle::BackgroundCircle;
use crate::background_square::BackgroundSquare;
use rand::{prelude::ThreadRng, Rng};
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;

#[styled_component(Background)]
pub fn background() -> Html {
    let mut rng = rand::thread_rng();

    let style = use_style(create_css());
    let mut background_elements = vec![];
    for _ in 0..5 {
        background_elements.push(html! { <BackgroundCircle
            size = {random_size(&mut rng)}
            x = {random_position(&mut rng)}
            y = {random_position(&mut rng)}
        /> });
        background_elements.push(html! {
            <BackgroundSquare
                size = {random_size(&mut rng)}
                x = {random_position(&mut rng)}
                y = {random_position(&mut rng)}
            />
        })
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

fn random_size(rng: &mut ThreadRng) -> u8 {
    rng.gen_range(0..20)
}

fn random_position(rng: &mut ThreadRng) -> u8 {
    rng.gen_range(0..100)
}
