mod ability_cell;
mod app;
mod components;
mod cursor_hit;
mod customkeys;
mod focus;
mod grid_layout;
mod grid_slot;
mod grid_templates;
mod icons;
mod inspector_detail;
mod navigation;
mod races;
mod system_hotkeys;
mod text;

use crate::app::App;

fn main() {
    dioxus::launch(App);
}
