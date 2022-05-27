use eyre::{bail, Result};
use yew::prelude::*;

use crate::logging::LogMessage;

#[function_component(App)]
pub fn app() -> Html {
    LogMessage::info("app loaded");
    if let Err(error) = always_fails() {
        LogMessage::error("error running the function that always errors", error);
    }

    html! {
      <div>
        <h1>{"Hello World"}</h1>
      </div>
    }
}

pub fn always_fails() -> Result<()> {
    bail!("I failed!")
}
