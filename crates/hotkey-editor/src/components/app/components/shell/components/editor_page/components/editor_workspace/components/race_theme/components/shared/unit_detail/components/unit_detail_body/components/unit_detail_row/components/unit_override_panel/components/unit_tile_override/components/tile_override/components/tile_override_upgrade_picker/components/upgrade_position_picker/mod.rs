pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use components::upgrade_position_picker_panel::UpgradePositionPickerPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use hooks::use_upgrade_position_picker;
use logic::UpgradePositionPickerShell;
use props::UpgradePositionPickerProps;
use style::CLASS;
use tw_macro::assert_component;

/// The upgraded-form position picker dialog. It owns its own dialog shell: the hook
/// shapes the grid config, the shell struct names the bordered panel, and this places
/// the panel inside its own backdrop `div` within the library `DialogRoot`. No project
/// class touches the library element — the backdrop is this component's own classed
/// `div`.
#[component]
pub fn UpgradePositionPicker(props: UpgradePositionPickerProps) -> Element {
    let model = use_upgrade_position_picker(&props);
    use_body_scroll_lock(model.open);
    let UpgradePositionPickerShell {
        open,
        on_open_change,
        title,
        on_close,
        explainer_text,
        grid_config,
    } = UpgradePositionPickerShell::from(&model);
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div {
                class: CLASS,
                UpgradePositionPickerPanel {
                    title,
                    on_close,
                    explainer_text,
                    grid_config,
                }
            }
        }
    }
}

assert_component!(UpgradePositionPicker);
