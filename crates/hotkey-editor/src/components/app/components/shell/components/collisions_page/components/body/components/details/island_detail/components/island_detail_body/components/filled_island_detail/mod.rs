pub mod components;
mod model;
mod presentation;
mod view;

pub use view::FilledIslandDetailView;
mod style;

use components::island_conflict_grid::IslandConflictGrid;
use components::island_detail_header::IslandDetailHeader;
use dioxus::prelude::*;
use model::FilledIslandDetailModel;
use presentation::FilledIslandDetailPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FilledIslandDetail(props: FilledIslandDetailModel) -> Element {
    let FilledIslandDetailPresentation {
        coordinate,
        count,
        conflicts,
    } = FilledIslandDetailPresentation::from(&props);
    rsx! {
        div {
            class: CLASS,
            IslandDetailHeader {
                coordinate,
                count,
            }
            IslandConflictGrid {
                conflicts,
            }
        }
    }
}

assert_component!(FilledIslandDetail);
