mod app;
mod components;
mod model;
mod services;

use crate::app::App;

fn main() {
    dioxus::launch(App);
}
