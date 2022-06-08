use super::bb_icon::BBIcon;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(TopMenu)]
pub fn top_menu() -> Html {
    html! {
      <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
        <div class="container-fluid">
            <div class="navbar-brand">
                <a href="#">
                    <img src="static/brooks.png" alt="Brooks Builds Icon" class={css!("height: 2rem;")} />
                </a>
                {"Brooks Builds"}
            </div>
        </div>
      </nav>
    }
}
