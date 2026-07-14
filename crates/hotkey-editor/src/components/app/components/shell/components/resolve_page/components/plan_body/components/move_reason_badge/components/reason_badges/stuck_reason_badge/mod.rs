mod model;
mod view;

pub use view::StuckReasonBadgeView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_badge::components::reason_badges::shared::reason_badge::ReasonBadge;
use dioxus::prelude::*;
use model::StuckReasonBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn StuckReasonBadge(props: StuckReasonBadgeModel) -> Element {
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

assert_component!(StuckReasonBadge);
