use yew::prelude::*;

use crate::logging::{LogLevel, LogMessage};

#[function_component(App)]
pub fn app() -> Html {
    let log = LogMessage::new("app loaded", LogLevel::Info);
    log.console().unwrap();
    log.send();

    html! {
      <div>
        <h1>{"Hello World"}</h1>
      </div>
    }
}
