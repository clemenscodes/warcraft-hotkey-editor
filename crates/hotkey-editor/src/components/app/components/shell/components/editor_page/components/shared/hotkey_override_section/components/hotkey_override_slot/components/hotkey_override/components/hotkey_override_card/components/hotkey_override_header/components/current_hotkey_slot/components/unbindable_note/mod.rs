mod model;
mod view;

pub use view::UnbindableNoteView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::UnbindableNoteModel;

#[component]
pub fn UnbindableNote(props: UnbindableNoteModel) -> Element {
    let text = props.text;
    rsx! {
        p {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(UnbindableNote);
