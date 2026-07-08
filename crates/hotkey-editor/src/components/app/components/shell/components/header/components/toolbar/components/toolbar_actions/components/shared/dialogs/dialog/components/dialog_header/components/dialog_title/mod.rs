mod props;
mod style;

use crate::components::app::components::shell::components::shared::gold_heading::{
    GoldHeading, GoldHeadingProps,
};
use dioxus::prelude::*;
pub use props::DialogTitleProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DialogTitle);

/// The dialog's heading. Keeps the `.dialog-title` h2 with its per-band sizing and
/// truncation; the loud gold look is the nested [`GoldHeading`].
#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let heading = GoldHeadingProps::from(&props);
    rsx! {
        h2 {
            class: CLASS,
            GoldHeading { ..heading }
        }
    }
}
