mod model;
mod view;

pub use view::UnbindableNoteView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::UnbindableNoteModel;

/// The muted note shown for a passive ability in place of a hotkey field.
#[component]
pub fn UnbindableNote(props: UnbindableNoteModel) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}

assert_component!(UnbindableNote);
