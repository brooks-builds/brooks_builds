use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <h1 data-test="landing-title" class="text-center">{"Welcome to Brooks Builds"}</h1>
    }
}
