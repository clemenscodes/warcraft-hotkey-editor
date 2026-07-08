mod props;
mod style;

use dioxus::prelude::*;
pub use props::HelpSectionTitleProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HelpSectionTitle);

/// A section heading inside the help guide: an `h3` wearing the uppercase gold
/// heading look, with its own per-band sizing.
#[component]
pub fn HelpSectionTitle(props: HelpSectionTitleProps) -> Element {
    let title = props.title;
    rsx! {
        h3 {
            class: CLASS,
            {title}
        }
    }
}
