use yew::prelude::*;

#[function_component(TopNavbar)]
pub fn top_navbar() -> Html {
    html! {
        <nav class="navbar navbar-expand-lg">
            <div class="container-fluid">
                <div>
                    <img class="navbar-brand" src="/static/brooks.png" alt="Brooks' logo" data-test="nav-logo" />
                    <span class="navbar-text" data-test="nav-title">{"Brooks Builds"}</span>
                </div>
            </div>
        </nav>
    }
}
