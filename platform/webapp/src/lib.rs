mod app;
mod logging;

use app::App;
use yew::start_app;

pub fn run() {
    start_app::<App>();
}