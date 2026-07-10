pub mod components;
mod props;
mod view;

pub use view::LayoutEditorContentView;
mod style;

use components::apply_button::ApplyButton;
use components::layout_grid::LayoutGrid;
use components::layout_intro::LayoutIntro;
use components::move_hotkey_toggle::MoveHotkeyToggle;
use dioxus::prelude::*;
use props::LayoutEditorContentProps;
use style::CLASS;
use tw_macro::assert_component;

/// The centered body column of the layout editor: the intro block above the
/// editable grid, the move-hotkey toggle, and the apply action.
#[component]
pub fn LayoutEditorContent(props: LayoutEditorContentProps) -> Element {
    let cells = props.cells;
    let toggle_checked = props.toggle_checked;
    let on_toggle = props.on_toggle;
    let on_apply = props.on_apply;
    rsx! {
        div {
            class: CLASS,
            LayoutIntro {}
            LayoutGrid { cells }
            MoveHotkeyToggle { checked: toggle_checked, on_toggle }
            ApplyButton { on_apply }
        }
    }
}

assert_component!(LayoutEditorContent);
