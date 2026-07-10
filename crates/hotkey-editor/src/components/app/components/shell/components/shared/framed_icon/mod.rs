pub mod components;
mod logic;
mod props;
mod state;

use components::card_glow_icon::{CardGlowIcon, CardGlowIconProps};
use components::control_plain_icon::{ControlPlainIcon, ControlPlainIconProps};
use components::placeholder_icon::{PlaceholderIcon, PlaceholderIconProps};
use components::tile_glow_icon::{TileGlowIcon, TileGlowIconProps};
use components::tile_plain_icon::{TilePlainIcon, TilePlainIconProps};
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
            let icon = TilePlainIconProps::from(&props);
            rsx! {
                TilePlainIcon { ..icon }
            }
        }
        FramedIconStyle::TileGlow => {
            let icon = TileGlowIconProps::from(&props);
            rsx! {
                TileGlowIcon { ..icon }
            }
        }
        FramedIconStyle::ControlPlain => {
            let icon = ControlPlainIconProps::from(&props);
            rsx! {
                ControlPlainIcon { ..icon }
            }
        }
        FramedIconStyle::CardGlow => {
            let icon = CardGlowIconProps::from(&props);
            rsx! {
                CardGlowIcon { ..icon }
            }
        }
        FramedIconStyle::Placeholder => {
            let icon = PlaceholderIconProps::from(&props);
            rsx! {
                PlaceholderIcon { ..icon }
            }
        }
    }
}

assert_component!(FramedIcon);
