mod props;
mod style;

use crate::components::app::components::shell::components::shared::gold_heading::{
    GoldHeading, GoldHeadingProps,
};
use dioxus::prelude::*;
pub use props::ToastTitleProps;
use tw_macro::assert_component;
assert_component!(ToastTitle);

/// The toast headline: the `.toast-title` div tints the text by toast type, and the
/// shared uppercase gold look is the nested [`GoldHeading`], which inherits that tint.
#[component]
pub fn ToastTitle(props: ToastTitleProps) -> Element {
    let class = style::class(props.toast_type);
    let heading = GoldHeadingProps::from(&props);
    rsx! {
        div {
            class,
            GoldHeading { ..heading }
        }
    }
}
