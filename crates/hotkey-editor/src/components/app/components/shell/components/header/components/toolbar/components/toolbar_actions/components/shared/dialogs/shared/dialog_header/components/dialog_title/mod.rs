mod props;
mod style;

use dioxus::prelude::*;
use props::DialogTitleProps;
use style::CLASS;
use tw_macro::assert_component;

/// The dialog's heading: an `h2` wearing the uppercase gold heading look, with its
/// own per-band sizing and mobile/tablet truncation.
#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let title = props.title;
    rsx! {
        h2 {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(DialogTitle);
