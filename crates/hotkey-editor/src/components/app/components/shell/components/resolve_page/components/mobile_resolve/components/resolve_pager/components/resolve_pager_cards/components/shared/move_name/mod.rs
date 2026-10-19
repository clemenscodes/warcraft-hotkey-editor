pub mod components;
mod model;
mod view;

pub use view::MoveNameView;

use components::link_move_name::LinkMoveName;
use components::plain_move_name::PlainMoveName;
use dioxus::prelude::*;
use model::MoveNameModel;
use tw_macro::assert_component;

#[component]
pub fn MoveName(props: MoveNameModel) -> Element {
    match props.is_link {
        true => {
            let text = props.text;
            rsx! {
                LinkMoveName {
                    text,
                }
            }
        }
        false => {
            let text = props.text;
            rsx! {
                PlainMoveName {
                    text,
                }
            }
        }
    }
}

assert_component!(MoveName);
