use super::components::top_menu::TopMenu;
use crate::router::switch;
use crate::router::Route;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
      <BrowserRouter>
        <div class="container-fluid">
          <TopMenu />
          <Switch<Route> render={Switch::render(switch)} />
        </div>
      </BrowserRouter>
    }
}
