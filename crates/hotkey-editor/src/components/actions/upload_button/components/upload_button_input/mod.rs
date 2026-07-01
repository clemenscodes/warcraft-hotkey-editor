mod props;
mod style;

use crate::assert_component;
use crate::services::files::upload::UPLOAD_INPUT_ELEMENT_ID;
use dioxus::prelude::*;
pub use props::UploadButtonInputProps;
use style::CLASS;
assert_component!(UploadButtonInput);

/// The visually hidden `<input type="file">` that receives the CustomKeys.txt.
#[component]
pub fn UploadButtonInput(props: UploadButtonInputProps) -> Element {
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
