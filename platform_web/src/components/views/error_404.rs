use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Error404)]
pub fn error_404() -> Html {
    html! {
        <div class="container text-center">
            <div class="row">
                <h1 data-test="404-title">{"404"}</h1>
            </div>
            <div class="row">
                <h2 data-test="404-subtitle">{"We couldn't find the page you're looking for"}</h2>
            </div>
            <div class="row" data-test="404-home-link">
                <Link<Route> to={Route::Home}>{"Take me home"}</Link<Route>>
            </div>
        </div>
    }
}
