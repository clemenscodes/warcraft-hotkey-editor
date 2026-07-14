pub mod components;
mod frame;
mod model;
pub mod presentation;
mod view;

pub use view::ResolvePageView;

mod style;

use components::clear_state::ClearState;
use components::empty_state::EmptyState;
use components::plan_body::PlanBodyView;
use components::resolve_plan_header::ResolvePlanHeaderView;
use dioxus::prelude::*;
use dioxus_kit::frame::Page;
use frame::ResolvePageFrame;
use model::ResolvePageModel;
use presentation::{ResolvePagePresentation, ResolvePlanPresentation, use_resolve_page};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ResolvePage(props: ResolvePageModel) -> Element {
    let plan = match use_resolve_page(&props) {
        ResolvePagePresentation::NoFile => {
            return rsx! {
                EmptyState {}
            };
        }
        ResolvePagePresentation::Clear => {
            return rsx! {
                ClearState {}
            };
        }
        ResolvePagePresentation::Plan(plan) => *plan,
    };
    let ResolvePlanPresentation {
        moves_text,
        unresolved_count,
        running,
        on_apply,
        breadcrumbs,
        section,
        unresolved,
    } = plan;
    let header = ResolvePlanHeaderView {
        moves_text,
        unresolved_count,
        running,
        on_apply,
        breadcrumbs,
    };
    let body = PlanBodyView {
        section,
        unresolved,
    };
    let frame = ResolvePageFrame { header, body };
    rsx! {
        Page {
            class: CLASS,
            frame,
        }
    }
}

assert_component!(ResolvePage);
