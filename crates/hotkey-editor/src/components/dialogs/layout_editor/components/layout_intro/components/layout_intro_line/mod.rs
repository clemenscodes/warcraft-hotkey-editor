mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::LayoutIntroLineProps;

assert_component!(LayoutIntroLine);

/// A single instruction line in the layout editor's intro.
#[component]
pub fn LayoutIntroLine(props: LayoutIntroLineProps) -> Element {
    let line = props.line;
    rsx! {
        p {
            class: CLASS,
            {line}
        }
    }
}
