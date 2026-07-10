mod props;
mod style;

use dioxus::prelude::*;
pub use props::LayoutIntroLineProps;
use style::CLASS;
use tw_macro::assert_component;

/// A single instruction line in the layout editor's intro.
#[component]
pub fn LayoutIntroLine(props: LayoutIntroLineProps) -> Element {
    let line = props.line;
    rsx! {
        p { class: CLASS, {line} }
    }
}

assert_component!(LayoutIntroLine);
