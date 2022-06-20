use crate::router::Route;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[styled_component(TopMenu)]
pub fn top_menu() -> Html {
    html! {
      <nav class="h-8">
        <div class="grid grid-cols-3 gap-4 px-5">
            <div class="flex items-center">
                <span data-test="top-menu-logo">
                    <Link<Route> to={ Route::Home }>
                      <img src="static/brooks.png" alt="Brooks Builds Icon" class="h-8" />
                    </Link<Route>>
                </span>
                {"Brooks Builds"}
            </div>
            <div class="flex justify-center">
              {"I am in the middle"}
            </div>
            <div class="flex justify-end">
              {"right side"}
            </div>
        </div>
      </nav>
    }
}
