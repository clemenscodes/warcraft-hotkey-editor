mod hooks;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::shared::alt_position_picker_body::AltPositionPickerBody;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use hooks::use_alt_position_picker;
use logic::AltPositionPickerShell;
pub use props::AltPositionPickerProps;
use style::{CLASS, OVERLAY};
use tw_macro::assert_component;

assert_component!(AltPositionPicker);

/// The off-state position picker dialog. It owns its own dialog shell: the hook
/// shapes the grid config, the shell struct names the header and scroll body, and
/// this places them inside the backdrop and bordered box.
#[component]
pub fn AltPositionPicker(props: AltPositionPickerProps) -> Element {
    let model = use_alt_position_picker(&props);
    use_body_scroll_lock(model.open);
    let AltPositionPickerShell {
        open,
        on_open_change,
        header,
        body,
    } = AltPositionPickerShell::from(&model);
    rsx! {
        DialogRoot {
            class: OVERLAY,
            open,
            on_open_change,
            DialogContent {
                class: CLASS.to_library_class(),
                DialogHeader { ..header }
                AltPositionPickerBody { ..body }
            }
        }
    }
}
