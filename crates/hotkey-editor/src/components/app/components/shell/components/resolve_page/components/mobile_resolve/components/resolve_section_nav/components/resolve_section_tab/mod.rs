pub mod components;
mod model;
mod presentation;
mod view;

pub use view::ResolveSectionTabView;

use crate::components::app::components::shell::components::resolve_page::presentation::MoveCategory;
use components::fight_section_tab::FightSectionTab;
use components::gap_pull_section_tab::GapPullSectionTab;
use components::spill_section_tab::SpillSectionTab;
use components::swap_section_tab::SwapSectionTab;
use dioxus::prelude::*;
use model::ResolveSectionTabModel;
use presentation::use_resolve_section_tab;
use tw_macro::assert_component;

#[component]
pub fn ResolveSectionTab(props: ResolveSectionTabModel) -> Element {
    let category = use_resolve_section_tab(&props);
    let label = props.label.clone();
    let count = props.count;
    let active = props.active;
    let onclick = props.onclick;
    match category {
        MoveCategory::Fight => rsx! {
            FightSectionTab {
                label,
                count,
                active,
                onclick,
            }
        },
        MoveCategory::GapPull => rsx! {
            GapPullSectionTab {
                label,
                count,
                active,
                onclick,
            }
        },
        MoveCategory::Spill => rsx! {
            SpillSectionTab {
                label,
                count,
                active,
                onclick,
            }
        },
        MoveCategory::Swap => rsx! {
            SwapSectionTab {
                label,
                count,
                active,
                onclick,
            }
        },
    }
}

assert_component!(ResolveSectionTab);
