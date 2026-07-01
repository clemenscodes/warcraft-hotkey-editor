use super::components::carriers_dialog::CarriersDialog;
use super::components::resolve_breadcrumbs::ResolveBreadcrumbs;
use super::components::resolve_breadcrumbs::components::resolve_breadcrumb::ResolveBreadcrumbProps;
use super::components::resolve_clear_state::ResolveClearState;
use super::components::resolve_empty_state::ResolveEmptyState;
use super::components::resolve_move_row::ResolveMoveRowProps;
use super::components::resolve_plan_body::{ResolvePlanBody, ResolvePlanBodySection};
use super::components::resolve_plan_header::{ResolvePlanHeader, ResolvePlanHeaderProps};
use super::components::resolve_unresolved_row::ResolveUnresolvedRowProps;
use super::logic::ResolvePlanView;
use super::props::ResolvePageProps;
use super::style::CLASS;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use gloo_timers::future::TimeoutFuture;
use warcraft_keybinds::CustomKeys;

/// Computes the cascade preview (memoised on the loaded keys), wires the Apply
/// handler and carriers dialog, and shapes the active section, breadcrumbs, and
/// header — returning the fully rendered page for the current state (no-file,
/// all-clear, or the plan).
pub(super) fn use_resolve_page(props: &ResolvePageProps) -> Element {
    let mut loaded_keys = props.loaded_keys;
    let view_navigation = use_context::<ViewNavigationContext>();
    let toast_api = use_toast();
    let mut is_running = use_signal(|| false);
    let carriers_dialog = use_signal(|| None::<super::logic::CarriersDialogData>);
    let mut selected_move_category = props.selected_move_category;
    let plan_memo = use_memo(move || {
        let guard = loaded_keys.read();
        guard.as_ref().map(ResolvePlanView::build)
    });
    let has_file = loaded_keys.read().is_some();
    let plan_option = plan_memo();
    let move_count = plan_option.as_ref().map(|view| view.move_count()).unwrap_or(0);
    let unresolved_count = plan_option
        .as_ref()
        .map(|view| view.unresolved.len())
        .unwrap_or(0);
    let running_now = *is_running.read();
    let dialog_state = carriers_dialog.read().clone();
    let handle_apply = move |_event: MouseEvent| {
        if *is_running.read() {
            return;
        }
        let working_copy: CustomKeys = {
            let read_guard = loaded_keys.read();
            let Some(file) = read_guard.as_ref() else {
                return;
            };
            file.clone()
        };
        is_running.set(true);
        spawn(async move {
            TimeoutFuture::new(0).await;
            let mut working_copy = working_copy;
            let plan = working_copy.resolve_conflicts();
            let move_count = plan.move_count();
            let unresolved_count = plan.unresolved_count();
            let normalized = working_copy.normalize();
            loaded_keys.set(Some(normalized));
            is_running.set(false);
            let summary = if unresolved_count == 0 {
                format!("Moved {move_count} ability slot(s). No remaining conflicts.")
            } else {
                format!("Moved {move_count} ability slot(s). {unresolved_count} could not be placed.")
            };
            let title = String::from("Cascade applied");
            let toast_options = ToastOptions::new().description(summary);
            toast_api.success(title, toast_options);
        });
    };
    if !has_file {
        return rsx! {
            ResolveEmptyState {}
        };
    }
    if move_count == 0 && unresolved_count == 0 {
        return rsx! {
            ResolveClearState {}
        };
    }
    let plan = plan_option.expect("plan present when a file is loaded");
    let selected_slug = selected_move_category.read().clone();
    let active = plan.active_section(selected_slug.as_deref());
    let active_category = active.map(|section| section.category);
    let mut breadcrumbs: Vec<ResolveBreadcrumbProps> = Vec::with_capacity(plan.sections.len());
    for section in &plan.sections {
        let category = section.category;
        let is_active = active_category == Some(category);
        let data_breadcrumb = category.data_breadcrumb();
        let title = section.title.to_owned();
        let count = section.moves.len();
        let mut selection = selected_move_category;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            let slug = category.data_breadcrumb().to_owned();
            selection.set(Some(slug));
        });
        let breadcrumb = ResolveBreadcrumbProps {
            title,
            count,
            data_breadcrumb,
            active: is_active,
            onclick,
        };
        breadcrumbs.push(breadcrumb);
    }
    let section = active.map(|section| {
        let rows: Vec<ResolveMoveRowProps> = section
            .moves
            .iter()
            .map(|move_view| ResolveMoveRowProps {
                move_view: move_view.clone(),
                view_navigation,
                carriers_dialog,
            })
            .collect();
        ResolvePlanBodySection {
            data_category: section.category.data_breadcrumb(),
            rows,
        }
    });
    let unresolved_rows: Vec<ResolveUnresolvedRowProps> = plan
        .unresolved
        .iter()
        .map(|unresolved_view| ResolveUnresolvedRowProps {
            unresolved_view: unresolved_view.clone(),
            carriers_dialog,
        })
        .collect();
    let move_noun = if move_count == 1 { "move" } else { "moves" };
    let moves_text = format!("{move_count} {move_noun}");
    let apply_handler = EventHandler::new(handle_apply);
    let header = ResolvePlanHeaderProps {
        moves_text,
        unresolved_count,
        running: running_now,
        on_apply: apply_handler,
    };
    rsx! {
        section {
            class: CLASS,
            "data-resolve-state": "plan",
            "data-move-count": "{move_count}",
            "data-unresolved-count": "{unresolved_count}",
            ResolvePlanHeader { ..header }
            ResolveBreadcrumbs { breadcrumbs }
            ResolvePlanBody { section, unresolved_rows }
        }
        if let Some(dialog_data) = dialog_state {
            CarriersDialog { dialog_data, carriers_dialog, view_navigation }
        }
    }
}
