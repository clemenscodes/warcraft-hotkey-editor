mod props;
mod view;

pub use view::HelpSectionTitleView;
mod style;

use dioxus::prelude::*;
use props::HelpSectionTitleProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(HelpSectionTitle);
