pub mod components;
mod hooks;
pub mod logic;
mod props;
mod style;

use crate::assert_component;
use components::carriers_dialog_host::CarriersDialogHost;
use components::resolve_breadcrumbs::ResolveBreadcrumbs;
use components::resolve_clear_state::ResolveClearState;
use components::resolve_empty_state::ResolveEmptyState;
use components::resolve_plan_body::ResolvePlanBody;
use components::resolve_plan_header::ResolvePlanHeader;
use dioxus::prelude::*;
use hooks::{ResolvePageView, use_resolve_page};
pub use props::ResolvePageProps;
use style::CLASS;

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
                ResolveEmptyState {}
            };
        }
        ResolvePageView::Clear => {
            return rsx! {
                ResolveClearState {}
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
            ResolvePlanHeader { ..plan.header }
            ResolveBreadcrumbs { ..plan.breadcrumbs }
            ResolvePlanBody { ..plan.body }
            CarriersDialogHost { ..plan.carriers_dialog_host }
        }
    }
}
