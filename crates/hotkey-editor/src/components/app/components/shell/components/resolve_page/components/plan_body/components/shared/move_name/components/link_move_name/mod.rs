mod props;
mod style;

use dioxus::prelude::*;
pub use props::LinkMoveNameProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(LinkMoveName);

/// The clickable ability name that deep-links into the editor; underlines on the button's hover.
#[component]
pub fn LinkMoveName(props: LinkMoveNameProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}
