use super::cell::KeyColumn;
use super::key_event::BrowserKeyEvent;
use super::model::KeyPickerBoardModel;
use dioxus::prelude::*;
use warcraft_keybinds::KeyCode;

pub(super) struct KeyPickerBoardPresentation {
    pub(super) columns: Vec<KeyColumn>,
    pub(super) on_pick: EventHandler<KeyCode>,
    pub(super) onkeydown: EventHandler<Event<KeyboardData>>,
}

impl From<&KeyPickerBoardModel> for KeyPickerBoardPresentation {
    fn from(props: &KeyPickerBoardModel) -> Self {
        let on_pick = props.on_pick;
        let on_close = props.on_close;
        let columns = props.columns.clone();
        let mut pickable_codes: Vec<KeyCode> = Vec::new();
        for column in &columns {
            let column_codes = column.pickable_codes();
            pickable_codes.extend(column_codes);
        }
        let onkeydown = EventHandler::new(move |event: Event<KeyboardData>| {
            event.stop_propagation();
            let key_value = event.data().key().to_string();
            if key_value == "Escape" {
                event.prevent_default();
                on_close.call(());
                return;
            }
            let code_value = event.data().code().to_string();
            let browser_event = BrowserKeyEvent::new(&key_value, &code_value);
            let Some(resolved) = browser_event.pick_among(&pickable_codes) else {
                return;
            };
            event.prevent_default();
            on_pick.call(resolved);
        });
        Self {
            columns,
            on_pick,
            onkeydown,
        }
    }
}

impl ddd::Presentation for KeyPickerBoardPresentation {
    type Model = KeyPickerBoardModel;
}
