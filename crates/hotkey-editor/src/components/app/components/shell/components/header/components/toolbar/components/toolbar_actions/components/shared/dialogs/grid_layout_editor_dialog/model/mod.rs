use super::view::GridLayoutEditorDialogView;
use dioxus::prelude::*;

/// What the grid-layout editor needs: the open value it drives and the change handler
/// mirroring the headless dialog's own close back to the trigger that owns the open signal.
/// The grid, preferences, and nested picker are self-sourced from context, so nothing else
/// is threaded.
#[derive(Props, Clone, PartialEq)]
pub struct GridLayoutEditorDialogModel {
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl From<&GridLayoutEditorDialogView> for GridLayoutEditorDialogModel {
    fn from(view: &GridLayoutEditorDialogView) -> Self {
        let GridLayoutEditorDialogView {
            open,
            on_open_change,
        } = view.clone();
        Self {
            open,
            on_open_change,
        }
    }
}

impl ddd::Model for GridLayoutEditorDialogModel {
    type View = GridLayoutEditorDialogView;
}
