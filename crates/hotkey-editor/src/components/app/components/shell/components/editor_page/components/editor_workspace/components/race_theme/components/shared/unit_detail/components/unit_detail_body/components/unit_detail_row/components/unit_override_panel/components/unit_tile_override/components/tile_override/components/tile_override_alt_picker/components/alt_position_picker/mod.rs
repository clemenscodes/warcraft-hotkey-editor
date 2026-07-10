pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use components::alt_position_picker_panel::AltPositionPickerPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use hooks::use_alt_position_picker;
use logic::AltPositionPickerShell;
pub use props::AltPositionPickerProps;
use style::CLASS;
use tw_macro::assert_component;

assert_component!(AltPositionPicker);

/// The off-state position picker dialog. It owns its own dialog shell: the hook shapes
/// the grid config, the shell struct names the bordered panel, and this places the
/// panel inside its own backdrop `div` within the library `DialogRoot`. No project
/// class touches the library element — the backdrop is this component's own classed
/// `div`.
#[component]
pub fn AltPositionPicker(props: AltPositionPickerProps) -> Element {
    let model = use_alt_position_picker(&props);
    use_body_scroll_lock(model.open);
    let AltPositionPickerShell {
        open,
        on_open_change,
        panel,
    } = AltPositionPickerShell::from(&model);
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                AltPositionPickerPanel { ..panel }
            }
        }
    }
}
