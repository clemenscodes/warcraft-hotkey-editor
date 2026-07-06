mod props;
mod style;

use dioxus::prelude::*;
pub use props::DialogFooterProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DialogFooter);

/// The pinned action bar below the scrolling body. A dialog with an action footer
/// hands it content; a dialog without one hands it `None`, and nothing renders.
#[component]
pub fn DialogFooter(props: DialogFooterProps) -> Element {
    let Some(content) = props.footer.clone() else {
        return rsx! {};
    };
    rsx! {
        footer { class: CLASS, {content} }
    }
}
