pub mod components;
mod hooks;
pub mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::shared::breadcrumbs::Breadcrumbs;
use components::clear_state::ClearState;
use components::empty_state::EmptyState;
use components::plan_body::PlanBody;
use components::plan_header::PlanHeader;
use dioxus::prelude::*;
use hooks::{ResolvePageView, use_resolve_page};
pub use props::ResolvePageProps;
use style::CLASS;
use tw_macro::assert_component;

assert_component!(ResolvePage);

/// The Resolve page: a transparent preview of the cascade plan — every move the
/// algorithm would make and any unresolved abilities — with an Apply button that
/// runs the cascade. Shows an upload prompt with no file and an all-clear state
/// when there is nothing to resolve. The hook shapes the state's data; the body
/// only places it.
#[component]
pub fn ResolvePage(props: ResolvePageProps) -> Element {
    let plan = match use_resolve_page(&props) {
        ResolvePageView::NoFile => {
            return rsx! {
                EmptyState {}
            };
        }
        ResolvePageView::Clear => {
            return rsx! {
                ClearState {}
            };
        }
        ResolvePageView::Plan(plan) => *plan,
    };
    let move_count = plan.move_count;
    let unresolved_count = plan.unresolved_count;
    rsx! {
        section {
            class: CLASS,
            "data-resolve-state": "plan",
            "data-move-count": "{move_count}",
            "data-unresolved-count": "{unresolved_count}",
            PlanHeader { ..plan.header }
            Breadcrumbs { ..plan.breadcrumbs }
            PlanBody { ..plan.body }
        }
    }
}
