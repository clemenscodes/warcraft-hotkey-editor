use super::KeyPickerBody;
use super::model::KeyPickerBodyModel;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use browser_kit::frame::Render;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

/// The published `View` contract mirroring [`KeyPickerBodyModel`], threaded to this component
/// as data. It is also the key picker dialog's body region: it `impl Render` and renders the
/// presentational `KeyPickerBody` once, so the dialog places the published `View` directly as
/// `WarcraftDialog`'s body, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct KeyPickerBodyView {
    pub columns: Vec<KeyColumn>,
    pub on_pick: EventHandler<KeyCode>,
    pub on_close: EventHandler<()>,
}

impl ddd::View for KeyPickerBodyView {}

impl Render for KeyPickerBodyView {
    type Model = KeyPickerBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let columns = self.columns.clone();
        let on_pick = self.on_pick;
        let on_close = self.on_close;
        rsx! {
            KeyPickerBody {
                columns,
                on_pick,
                on_close,
            }
        }
    }
}
