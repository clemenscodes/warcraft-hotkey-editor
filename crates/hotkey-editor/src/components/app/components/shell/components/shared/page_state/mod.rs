mod props;
mod style;

use dioxus::prelude::*;
pub use props::PageStateProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PageState);

/// The centered, vertically-filling section that hosts a page's empty or all-clear
/// message. It owns the shared layout — a column centered in the available space —
/// and nothing page-specific: each page wraps it in a thin identity element that
/// carries the page's own data attributes and passes the icon/label/message leaves
/// as children.
#[component]
pub fn PageState(props: PageStateProps) -> Element {
    let children = props.children;
    rsx! {
        section {
            class: CLASS,
            {children}
        }
    }
}
