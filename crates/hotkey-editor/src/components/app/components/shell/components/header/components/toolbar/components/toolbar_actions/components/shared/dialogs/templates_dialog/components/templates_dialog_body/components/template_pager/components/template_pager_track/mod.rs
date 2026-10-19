pub mod components;
mod model;
mod view;

pub use view::TemplatePagerTrackView;
mod style;

use components::template_pager_card_host::TemplatePagerCardHost;
use dioxus::prelude::*;
use model::TemplatePagerTrackModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TemplatePagerTrack(props: TemplatePagerTrackModel) -> Element {
    let cards = props.cards;
    rsx! {
        div {
            class: CLASS,
            for card in cards {
                TemplatePagerCardHost {
                    key: "{card.name}",
                    name: card.name.clone(),
                    description: card.description.clone(),
                    resolved: card.resolved.clone(),
                    on_apply: card.on_apply,
                }
            }
        }
    }
}

assert_component!(TemplatePagerTrack);
