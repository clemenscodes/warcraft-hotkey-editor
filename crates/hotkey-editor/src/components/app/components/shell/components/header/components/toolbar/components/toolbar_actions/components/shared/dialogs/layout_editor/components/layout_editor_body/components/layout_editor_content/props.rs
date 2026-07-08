use super::components::apply_button::ApplyButtonProps;
use super::components::layout_grid::LayoutGridProps;
use super::components::move_hotkey_toggle::MoveHotkeyToggleProps;
use dioxus::prelude::*;

/// The centered column's inputs: the editable grid, the move-hotkey toggle, and
/// the apply action. The intro block takes no data. Each is spread into its own
/// typed child, so no markup is ever threaded through as a value.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutEditorContentProps {
    pub grid: LayoutGridProps,
    pub toggle: MoveHotkeyToggleProps,
    pub apply: ApplyButtonProps,
}

impl From<&LayoutEditorContentProps> for LayoutGridProps {
    fn from(props: &LayoutEditorContentProps) -> Self {
        props.grid.clone()
    }
}

impl From<&LayoutEditorContentProps> for MoveHotkeyToggleProps {
    fn from(props: &LayoutEditorContentProps) -> Self {
        props.toggle.clone()
    }
}

impl From<&LayoutEditorContentProps> for ApplyButtonProps {
    fn from(props: &LayoutEditorContentProps) -> Self {
        props.apply.clone()
    }
}
