mod app;
mod components;
mod customkeys;
mod domain;
mod focus;
mod navigation;
mod system_hotkeys;
mod text;

use crate::app::App;

fn main() {
    dioxus::launch(App);
}
