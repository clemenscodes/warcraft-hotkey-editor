use super::KeyPickerBody;
use super::model::KeyPickerBodyModel;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use browser_kit::frame::Render;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

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
