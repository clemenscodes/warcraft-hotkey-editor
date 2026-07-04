mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::DialogTitleProps;
use style::CLASS;
assert_component!(DialogTitle);

/// The dialog's heading text. A leaf: the header passes the title as children.
#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let title = props.children.clone();
    rsx! {
        h2 { class: CLASS, {title} }
    }
}
