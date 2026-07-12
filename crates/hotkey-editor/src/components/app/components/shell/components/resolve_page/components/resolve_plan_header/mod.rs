pub mod components;
mod model;
mod view;

pub use view::ResolvePlanHeaderView;

use crate::components::app::components::shell::components::shared::breadcrumbs::Breadcrumbs;
use components::plan_header::PlanHeader;
use dioxus::prelude::*;
use model::ResolvePlanHeaderModel;
use tw_macro::assert_component;

/// The resolve page's header: the plan summary row with its Apply control above the
/// move-category breadcrumb tabs. It composes the plan header and the shared `Breadcrumbs`
/// (named for the move categories) as the single header region the frame places, so both sit
/// above the scrollable plan body.
#[component]
pub fn ResolvePlanHeader(props: ResolvePlanHeaderModel) -> Element {
    let moves_text = props.moves_text;
    let unresolved_count = props.unresolved_count;
    let running = props.running;
    let on_apply = props.on_apply;
    let breadcrumbs = props.breadcrumbs;
    rsx! {
        PlanHeader {
            moves_text,
            unresolved_count,
            running,
            on_apply,
        }
        Breadcrumbs {
            breadcrumbs,
            aria_label: "Move categories",
        }
    }
}

assert_component!(ResolvePlanHeader);
