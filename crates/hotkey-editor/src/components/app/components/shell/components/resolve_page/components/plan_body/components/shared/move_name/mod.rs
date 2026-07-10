pub mod components;
mod props;
mod view;

pub use view::MoveNameView;

use components::link_move_name::LinkMoveName;
use components::plain_move_name::PlainMoveName;
use dioxus::prelude::*;
use props::MoveNameProps;
use tw_macro::assert_component;

/// A moved/rival ability's name. A dispatcher: from whether it links to an owning unit
/// it renders the clickable `LinkMoveName` xor the `PlainMoveName`; there is no
/// `data-link` attribute.
#[component]
pub fn MoveName(props: MoveNameProps) -> Element {
    match props.is_link {
        true => {
            let text = props.text;
            rsx! {
                LinkMoveName { text }
            }
        }
        false => {
            let text = props.text;
            rsx! {
                PlainMoveName { text }
            }
        }
    }
}

assert_component!(MoveName);
