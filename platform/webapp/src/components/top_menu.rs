use super::bb_icon::BBIcon;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(TopMenu)]
pub fn top_menu() -> Html {
    let style = css!(
        r#"
            height: 1.5rem;
            background-color: RGB(173, 173, 173);
            display: flex;
            justify-content: space-between;
        "#
    );

    html! {
      <div class={classes!(style, "container")}>
            <section class="left flex align-center">
                <BBIcon alt="Brooks Builds Icon" src="static/brooks.png" width=20.0 height=20.0 />
            </section>
            <section class="middle"></section>
            <section class="right"></section>
      </div>
    }
}
