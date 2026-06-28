mod logic;
mod props;
mod style;

use dioxus::prelude::*;
use dioxus_primitives::toast::use_toast;

use crate::components::command_grid::components::{CommandGrid, CommandGridHeading};
use logic::SectionRender;
use style::COMMAND_GRID_SECTION_STYLES;
use warcraft_keybinds::GridBehavior;

pub(crate) use props::GridSectionProps;

#[component]
pub(crate) fn GridSection<B: GridBehavior>(props: GridSectionProps<B>) -> Element {
    let GridSectionProps { behavior, section } = props;
    let toast = use_toast();
    let SectionRender {
        heading,
        race,
        views,
        dragging_slot,
        drop_target_tile,
        drag_follower,
        on_select,
        on_activate,
        on_move,
        drop_blocked,
    } = SectionRender::new(behavior, &section, toast);

    rsx! {
        document::Stylesheet { href: COMMAND_GRID_SECTION_STYLES }
        div { 
            class: "grid-section",
            CommandGridHeading { heading }
            CommandGrid {
                views,
                grid_id: heading,
                race,
                dragging_slot,
                drop_target_tile,
                drag_follower,
                on_select,
                on_activate,
                on_move,
                drop_blocked,
            }
        }
    }
}
