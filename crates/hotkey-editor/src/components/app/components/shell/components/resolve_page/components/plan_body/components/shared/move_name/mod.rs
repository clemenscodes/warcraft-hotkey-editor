pub mod components;
mod logic;
mod props;

use components::link_move_name::{LinkMoveName, LinkMoveNameProps};
use components::plain_move_name::{PlainMoveName, PlainMoveNameProps};
use dioxus::prelude::*;
pub use props::MoveNameProps;
use tw_macro::assert_component;

/// A moved/rival ability's name. A dispatcher: from whether it links to an owning unit
/// it renders the clickable `LinkMoveName` xor the `PlainMoveName`; there is no
/// `data-link` attribute.
#[component]
pub fn MoveName(props: MoveNameProps) -> Element {
    match props.is_link {
        true => {
            let name = LinkMoveNameProps::from(&props);
            rsx! {
                LinkMoveName { ..name }
            }
        }
        false => {
            let name = PlainMoveNameProps::from(&props);
            rsx! {
                PlainMoveName { ..name }
            }
        }
    }
}

assert_component!(MoveName);
