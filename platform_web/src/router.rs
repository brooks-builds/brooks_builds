use crate::components::views::{auth_callback::AuthCallback, error_404::Error404, home::Home};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/auth/callback")]
    AuthCallback,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <Error404 /> },
        Route::AuthCallback => html! { <AuthCallback /> },
    }
}
