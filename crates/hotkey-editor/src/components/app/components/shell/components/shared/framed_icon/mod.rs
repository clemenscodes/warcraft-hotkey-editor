pub mod components;
mod icon_radius;
mod model;
mod view;

pub use view::FramedIconView;
mod state;

pub use icon_radius::IconRadius;

use components::card_glow_icon::CardGlowIcon;
use components::control_plain_icon::ControlPlainIcon;
use components::placeholder_icon::PlaceholderIcon;
use components::tile_glow_icon::TileGlowIcon;
use components::tile_plain_icon::TilePlainIcon;
use dioxus::prelude::*;
use model::FramedIconModel;
use state::FramedIconStyle;
use tw_macro::assert_component;

#[component]
pub fn FramedIcon(props: FramedIconModel) -> Element {
    let look = FramedIconStyle::from(&props);
    if props.src.is_none() && !props.placeholder {
        return rsx! {};
    }
    match look {
        FramedIconStyle::TilePlain => {
            let source = props.src.clone();
            let alt = props.alt.clone();
            rsx! {
                TilePlainIcon {
                    source,
                    alt,
                }
            }
        }
        FramedIconStyle::TileGlow => {
            let source = props.src.clone();
            let alt = props.alt.clone();
            rsx! {
                TileGlowIcon {
                    source,
                    alt,
                }
            }
        }
        FramedIconStyle::ControlPlain => {
            let source = props.src.clone();
            let alt = props.alt.clone();
            rsx! {
                ControlPlainIcon {
                    source,
                    alt,
                }
            }
        }
        FramedIconStyle::CardGlow => {
            let source = props.src.clone();
            let alt = props.alt.clone();
            rsx! {
                CardGlowIcon {
                    source,
                    alt,
                }
            }
        }
        FramedIconStyle::Placeholder => {
            let source = props.src.clone();
            let alt = props.alt.clone();
            rsx! {
                PlaceholderIcon {
                    source,
                    alt,
                }
            }
        }
    }
}

assert_component!(FramedIcon);
