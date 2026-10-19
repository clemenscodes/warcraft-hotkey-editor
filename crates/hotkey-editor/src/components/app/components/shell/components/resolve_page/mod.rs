pub mod components;
mod data;
mod model;
pub mod presentation;
mod view;

pub use view::ResolvePageView;

use crate::components::app::components::shell::components::shared::clear_state::ClearState;
use crate::components::app::components::shell::components::shared::empty_state::EmptyState;
use components::mobile_resolve::MobileResolve;
use dioxus::prelude::*;
use model::ResolvePageModel;
use presentation::{ResolvePagePresentation, ResolvePlanPresentation, use_resolve_page};
use tw_macro::assert_component;

#[component]
pub fn ResolvePage(props: ResolvePageModel) -> Element {
    let plan = match use_resolve_page(&props) {
        ResolvePagePresentation::NoFile => {
            return rsx! {
                EmptyState {
                    message: data::UPLOAD_PROMPT,
                }
            };
        }
        ResolvePagePresentation::Clear => {
            return rsx! {
                ClearState {
                    text: data::CLEAR_TEXT,
                }
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
    rsx! {
        MobileResolve {
            moves_text,
            unresolved_count,
            running,
            on_apply,
            breadcrumbs,
            section,
            unresolved,
        }
    }
}

assert_component!(ResolvePage);
