pub mod components;
mod model;
mod presentation;
mod view;

pub use view::HotkeyAltPositionPickerDialogView;
mod style;

use components::hotkey_alt_position_picker_panel::HotkeyAltPositionPickerPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use presentation::use_hotkey_alt_position_picker;
use presentation::HotkeyAltPositionPickerDialogShell;
use model::HotkeyAltPositionPickerDialogModel;
use style::CLASS;
use tw_macro::assert_component;

/// The off-state position picker dialog. It owns its own dialog shell: the hook shapes
/// the grid config, the shell struct names the bordered panel, and this places the
/// panel inside its own backdrop `div` within the library `DialogRoot`. No project
/// class touches the library element — the backdrop is this component's own classed
/// `div`.
#[component]
pub fn HotkeyAltPositionPickerDialog(props: HotkeyAltPositionPickerDialogModel) -> Element {
    let model = use_hotkey_alt_position_picker(&props);
    use_body_scroll_lock(model.open);
    let HotkeyAltPositionPickerDialogShell {
        open,
        on_open_change,
        title,
        on_close,
        explainer_text,
        grid_config,
    } = HotkeyAltPositionPickerDialogShell::from(&model);
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                HotkeyAltPositionPickerPanel {
                    title,
                    on_close,
                    explainer_text,
                    grid_config,
                }
            }
        }
    }
}

assert_component!(HotkeyAltPositionPickerDialog);
