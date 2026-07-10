pub mod components;
mod props;
mod style;

use components::apply_button::{ApplyButton, ApplyButtonProps};
use components::layout_grid::{LayoutGrid, LayoutGridProps};
use components::layout_intro::LayoutIntro;
use components::move_hotkey_toggle::{MoveHotkeyToggle, MoveHotkeyToggleProps};
use dioxus::prelude::*;
pub use props::LayoutEditorContentProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(LayoutEditorContent);

/// The centered body column of the layout editor: the intro block above the
/// editable grid, the move-hotkey toggle, and the apply action.
#[component]
pub fn LayoutEditorContent(props: LayoutEditorContentProps) -> Element {
    let grid = LayoutGridProps::from(&props);
    let toggle = MoveHotkeyToggleProps::from(&props);
    let apply = ApplyButtonProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            LayoutIntro {}
            LayoutGrid { ..grid }
            MoveHotkeyToggle { ..toggle }
            ApplyButton { ..apply }
        }
    }
}
