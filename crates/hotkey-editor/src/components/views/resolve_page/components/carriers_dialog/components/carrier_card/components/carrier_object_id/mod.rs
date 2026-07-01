mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CarrierObjectIdProps;
use style::CLASS;
assert_component!(CarrierObjectId);
#[component]
pub fn CarrierObjectId(props: CarrierObjectIdProps) -> Element {
    let text = props.text;
    rsx! { code { class: CLASS, {text} } }
}
