mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::LayoutIntroLineProps;
use style::CLASS;
assert_component!(LayoutIntroLine);

/// A single instruction line in the layout editor's intro.
#[component]
pub fn LayoutIntroLine(props: LayoutIntroLineProps) -> Element {
    let line = props.line;
    rsx! {
        p { class: CLASS, {line} }
    }
}
