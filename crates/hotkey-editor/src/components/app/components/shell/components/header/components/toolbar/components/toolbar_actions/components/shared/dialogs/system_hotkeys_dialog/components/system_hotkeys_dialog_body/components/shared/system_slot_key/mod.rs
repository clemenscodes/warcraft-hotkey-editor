pub mod components;
mod model;
mod view;

pub use view::SystemSlotKeyView;

use components::conflict_slot_key::ConflictSlotKey;
use components::plain_slot_key::PlainSlotKey;
use dioxus::prelude::*;
use model::SystemSlotKeyModel;
use tw_macro::assert_component;

#[component]
pub fn SystemSlotKey(props: SystemSlotKeyModel) -> Element {
    match props.conflict {
        false => {
            let label = props.label.clone();
            rsx! {
                PlainSlotKey {
                    label,
                }
            }
        }
        true => {
            let label = props.label.clone();
            rsx! {
                ConflictSlotKey {
                    label,
                }
            }
        }
    }
}

assert_component!(SystemSlotKey);
