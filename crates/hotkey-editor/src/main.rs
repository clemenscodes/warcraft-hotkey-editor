mod app;
mod components;
mod cursor_hit;
mod customkeys;
mod focus;
mod grid_layout;
mod grid_slot;
mod grid_templates;
mod icons;
mod navigation;
mod system_hotkeys;

use crate::app::App;

fn main() {
    dioxus::launch(App);
}
