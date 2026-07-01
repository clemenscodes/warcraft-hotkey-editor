mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::DialogTitleProps;

assert_component!(DialogTitle);

/// The dialog's heading text. A leaf: the header passes the title as children.
#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let title = props.children.clone();
    rsx! {
        h2 {
            class: CLASS,
            {title}
        }
    }
}
