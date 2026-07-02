mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictMoreProps;
use style::CLASS;
assert_component!(ConflictMore);
#[component]
pub fn ConflictMore(props: ConflictMoreProps) -> Element {
    let count = props.count;
    let onclick = props.onclick;
    rsx! {
        button { class: CLASS, r#type: "button", onclick, "+{count} more" }
    }
}
