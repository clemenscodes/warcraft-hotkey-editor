mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::DialogFooterProps;

assert_component!(DialogFooter);

/// The pinned action bar below the scrolling body. A dialog with an action footer
/// hands it content; a dialog without one hands it `None`, and nothing renders.
#[component]
pub fn DialogFooter(props: DialogFooterProps) -> Element {
    let Some(content) = props.footer.clone() else {
        return rsx! {};
    };
    rsx! {
        footer {
            class: CLASS,
            {content}
        }
    }
}
