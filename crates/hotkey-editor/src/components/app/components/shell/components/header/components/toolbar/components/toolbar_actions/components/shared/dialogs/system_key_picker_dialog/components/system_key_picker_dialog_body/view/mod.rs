use super::SystemKeyPickerDialogBody;
use super::model::SystemKeyPickerDialogBodyModel;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use browser_kit::frame::Render;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The published `View` contract mirroring [`SystemKeyPickerDialogBodyModel`], threaded to
/// this component as data. It is also the system key picker's body region: it `impl Render`
/// and renders the presentational `SystemKeyPickerDialogBody` once, so the dialog places the
/// published `View` directly as `WarcraftDialog`'s body, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct SystemKeyPickerDialogBodyView {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

impl ddd::View for SystemKeyPickerDialogBodyView {}

impl Render for SystemKeyPickerDialogBodyView {
    type Model = SystemKeyPickerDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let columns = self.columns.clone();
        let on_pick = self.on_pick;
        let on_close = self.on_close;
        rsx! {
            SystemKeyPickerDialogBody { columns, on_pick, on_close }
        }
    }
}
