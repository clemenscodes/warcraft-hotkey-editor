mod props;
mod view;

pub use view::LinkMoveNameView;
mod style;

use dioxus::prelude::*;
use props::LinkMoveNameProps;
use style::CLASS;
use tw_macro::assert_component;

/// The clickable ability name that deep-links into the editor; underlines on the button's hover.
#[component]
pub fn LinkMoveName(props: LinkMoveNameProps) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(LinkMoveName);
