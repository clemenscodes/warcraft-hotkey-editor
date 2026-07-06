mod props;
mod style;
use dioxus::prelude::*;
pub use props::CarrierObjectIdProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CarrierObjectId);
#[component]
pub fn CarrierObjectId(props: CarrierObjectIdProps) -> Element {
    let text = props.text;
    rsx! { code { class: CLASS, {text} } }
}
