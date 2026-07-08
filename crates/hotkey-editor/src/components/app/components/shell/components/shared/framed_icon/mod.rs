pub mod components;
mod props;
mod state;
mod style;

use components::framed_icon_image::{FramedIconImage, FramedIconImageProps};
use dioxus::prelude::*;
pub use props::{FramedIconProps, IconRadius};
use state::FramedIconStyle;
use tw_macro::assert_component;
assert_component!(FramedIcon);

/// The shared square-icon painter behind every ability and unit thumbnail across the
/// collisions, resolve, editor, and shared-dialog pages: a blue-bordered,
/// `object-cover` image that fills the box its parent hands it, with the radius,
/// hover glow, and empty-placeholder look chosen by typed props. Guarded — absent
/// `src` renders nothing, unless `placeholder` draws the empty framed square instead.
#[component]
pub fn FramedIcon(props: FramedIconProps) -> Element {
    let look = FramedIconStyle::from(&props);
    let class = style::class(look);
    let alt = props.alt;
    let Some(source) = props.src else {
        if props.placeholder {
            return rsx! {
                div { class }
            };
        }
        return rsx! {};
    };
    let image = FramedIconImageProps { source, alt };
    rsx! {
        div {
            class,
            FramedIconImage { ..image }
        }
    }
}
