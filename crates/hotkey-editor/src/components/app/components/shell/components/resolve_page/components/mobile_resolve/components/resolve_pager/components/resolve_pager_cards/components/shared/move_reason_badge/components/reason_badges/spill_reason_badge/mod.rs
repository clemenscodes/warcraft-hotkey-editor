mod model;
mod view;

pub use view::SpillReasonBadgeView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::mobile_resolve::components::resolve_pager::components::resolve_pager_cards::components::shared::move_reason_badge::components::reason_badges::shared::reason_badge::ReasonBadge;
use dioxus::prelude::*;
use model::SpillReasonBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SpillReasonBadge(props: SpillReasonBadgeModel) -> Element {
    let label = props.label;
    rsx! {
        span {
            class: CLASS,
            ReasonBadge {
                label,
            }
        }
    }
}

assert_component!(SpillReasonBadge);
