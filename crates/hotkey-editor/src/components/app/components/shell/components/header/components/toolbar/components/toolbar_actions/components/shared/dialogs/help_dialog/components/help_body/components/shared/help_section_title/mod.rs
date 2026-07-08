mod props;
mod style;

use crate::components::app::components::shell::components::shared::gold_heading::{
    GoldHeading, GoldHeadingProps,
};
use dioxus::prelude::*;
pub use props::HelpSectionTitleProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HelpSectionTitle);

/// A section heading inside the help guide. Keeps the `.help-section-title` h3 and
/// its per-band sizing; the shared uppercase gold look is the nested [`GoldHeading`].
#[component]
pub fn HelpSectionTitle(props: HelpSectionTitleProps) -> Element {
    let heading = GoldHeadingProps::from(&props);
    rsx! {
        h3 {
            class: CLASS,
            GoldHeading { ..heading }
        }
    }
}
