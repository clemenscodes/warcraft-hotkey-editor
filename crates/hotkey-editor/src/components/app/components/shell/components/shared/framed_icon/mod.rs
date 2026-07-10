pub mod components;
mod props;
mod state;

use components::card_glow_icon::CardGlowIcon;
use components::control_plain_icon::ControlPlainIcon;
use components::placeholder_icon::PlaceholderIcon;
use components::tile_glow_icon::TileGlowIcon;
use components::tile_plain_icon::TilePlainIcon;
use dioxus::prelude::*;
pub use props::{FramedIconProps, IconRadius};
use state::FramedIconStyle;
use tw_macro::assert_component;

/// The shared square-icon painter behind every ability and unit thumbnail across the
/// collisions, resolve, editor, and shared-dialog pages. A pure dispatcher: from the
/// resolved look it renders the matching per-look child — `TilePlainIcon` xor
/// `TileGlowIcon` xor `ControlPlainIcon` xor `CardGlowIcon` xor `PlaceholderIcon`. Each
/// look owns its own framed root and glow; this dispatcher only builds each look's props
/// from the shared `FramedIconProps` and renders the one the look selects. Guarded —
/// absent `src` renders nothing, unless `placeholder` draws the empty framed square.
#[component]
pub fn FramedIcon(props: FramedIconProps) -> Element {
    let look = FramedIconStyle::from(&props);
    if props.src.is_none() && !props.placeholder {
        return rsx! {};
    }
    match look {
        FramedIconStyle::TilePlain => {
            let source = props.src.clone();
            let alt = props.alt.clone();
            rsx! {
                TilePlainIcon { source, alt }
            }
        }
        FramedIconStyle::TileGlow => {
            let source = props.src.clone();
            let alt = props.alt.clone();
            rsx! {
                TileGlowIcon { source, alt }
            }
        }
        FramedIconStyle::ControlPlain => {
            let source = props.src.clone();
            let alt = props.alt.clone();
            rsx! {
                ControlPlainIcon { source, alt }
            }
        }
        FramedIconStyle::CardGlow => {
            let source = props.src.clone();
            let alt = props.alt.clone();
            rsx! {
                CardGlowIcon { source, alt }
            }
        }
        FramedIconStyle::Placeholder => {
            let source = props.src.clone();
            let alt = props.alt.clone();
            rsx! {
                PlaceholderIcon { source, alt }
            }
        }
    }
}

assert_component!(FramedIcon);
