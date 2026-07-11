mod model;
mod view;

pub use view::UploadButtonInputView;
mod style;

use crate::services::files::upload::UPLOAD_INPUT_ELEMENT_ID;
use dioxus::prelude::*;
use model::UploadButtonInputModel;
use style::CLASS;
use tw_macro::assert_component;

/// The visually hidden `<input type="file">` that receives the CustomKeys.txt.
#[component]
pub fn UploadButtonInput(props: UploadButtonInputModel) -> Element {
    let on_change = props.on_change;
    rsx! {
        input {
            id: UPLOAD_INPUT_ELEMENT_ID,
            class: CLASS,
            r#type: "file",
            accept: ".txt,text/plain",
            onchange: move |event| on_change.call(event),
        }
    }
}

assert_component!(UploadButtonInput);
