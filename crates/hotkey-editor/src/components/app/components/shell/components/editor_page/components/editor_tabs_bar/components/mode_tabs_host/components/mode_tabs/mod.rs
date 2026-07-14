pub mod components;
mod model;
mod presentation;
mod view;

pub use view::ModeTabsView;
mod style;

use components::mode_tab::ModeTab;
use dioxus::prelude::*;
use model::ModeTabsModel;
use presentation::{ModeTabBinding, ModeTabPair};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ModeTabs(props: ModeTabsModel) -> Element {
    let ModeTabPair { melee, campaign } = ModeTabPair::build(&props);
    let ModeTabBinding {
        label: melee_label,
        active: melee_active,
        onclick: melee_onclick,
        onkeydown: melee_onkeydown,
    } = melee;
    let ModeTabBinding {
        label: campaign_label,
        active: campaign_active,
        onclick: campaign_onclick,
        onkeydown: campaign_onkeydown,
    } = campaign;
    rsx! {
        div {
            class: CLASS,
            ModeTab {
                label: melee_label,
                active: melee_active,
                onclick: melee_onclick,
                onkeydown: melee_onkeydown,
            }
            ModeTab {
                label: campaign_label,
                active: campaign_active,
                onclick: campaign_onclick,
                onkeydown: campaign_onkeydown,
            }
        }
    }
}

assert_component!(ModeTabs);
