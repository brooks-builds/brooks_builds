mod app;
mod components;
mod logging;
mod pages;
mod router;

use app::App;
use yew::start_app;

pub fn run() {
    start_app::<App>();
}
