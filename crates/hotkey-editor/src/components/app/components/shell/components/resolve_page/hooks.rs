use super::components::breadcrumbs::BreadcrumbsProps;
use super::components::breadcrumbs::components::breadcrumb::BreadcrumbProps;
use super::components::carriers_dialog_host::CarriersDialogHostProps;
use super::components::plan_body::components::active_move_list::components::move_row::MoveRowProps;
use super::components::plan_body::components::unresolved_section::components::unresolved_row::UnresolvedRowProps;
use super::components::plan_body::{PlanBodyProps, PlanBodySection};
use super::components::plan_header::PlanHeaderProps;
use super::logic::{CarriersDialogData, PlanView};
use super::props::ResolvePageProps;
use crate::components::app::components::shell::components::toasts::{ToastOptions, use_toast};
use crate::services::customkeys::context::use_loaded_keys;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::{use_synced_route, use_view_navigation};
use crate::services::navigation::nav_snapshot::NavSnapshot;
use crate::services::resolve_selection::context::use_resolve_selection;
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use warcraft_keybinds::CustomKeys;

/// The three states the Resolve page renders, each as already shaped data. The
/// component body matches on this and places the data; the hook never builds
/// markup.
pub(super) enum ResolvePageView {
    /// No CustomKeys.txt is loaded yet — the upload prompt.
    NoFile,
    /// A file is loaded but has no conflicts — the all-clear state.
    Clear,
    /// A cascade plan to preview and apply.
    Plan(Box<ResolvePlanPresentation>),
}

/// Everything the plan state needs, fully shaped: the header, breadcrumb bar, and
/// body child props, the counts the root element tags itself with, and the
/// carriers-dialog host props.
pub(super) struct ResolvePlanPresentation {
    pub move_count: usize,
    pub unresolved_count: usize,
    pub header: PlanHeaderProps,
    pub breadcrumbs: BreadcrumbsProps,
    pub body: PlanBodyProps,
    pub carriers_dialog_host: CarriersDialogHostProps,
}

/// Computes the cascade preview (memoised on the loaded keys), wires the Apply
/// handler and the carriers dialog, and shapes the active section, breadcrumbs,
/// and header — returning the state's data for the body to render.
pub(super) fn use_resolve_page(props: &ResolvePageProps) -> ResolvePageView {
    let view_navigation = use_view_navigation();
    let resolve_selection = use_resolve_selection();
    let mut loaded_keys = use_loaded_keys();
    let toast_api = use_toast();
    let mut is_running = use_signal(|| false);
    let carriers_dialog = use_signal(|| None::<CarriersDialogData>);
    let selected_move_category = resolve_selection.selected_move_category;
    let mut synced_route = use_synced_route();
    let entry = props.entry.clone().filter(|value| !value.is_empty());
    use_effect(use_reactive!(|entry| {
        view_navigation.restore_view(AppView::Resolve);
        let mut selected = resolve_selection.selected_move_category;
        if *selected.peek() != entry {
            selected.set(entry.clone());
        }
        let snapshot = NavSnapshot::Resolve {
            entry: entry.clone(),
        };
        synced_route.set(snapshot);
    }));
    let plan_memo = use_memo(move || {
        let guard = loaded_keys.read();
        guard.as_ref().map(PlanView::build)
    });
    let has_file = loaded_keys.read().is_some();
    let plan_option = plan_memo();
    let move_count = plan_option
        .as_ref()
        .map(|view| view.move_count())
        .unwrap_or(0);
    let unresolved_count = plan_option
        .as_ref()
        .map(|view| view.unresolved.len())
        .unwrap_or(0);
    let running_now = *is_running.read();
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
                format!(
                    "Moved {move_count} ability slot(s). {unresolved_count} could not be placed."
                )
            };
            let title = String::from("Cascade applied");
            let toast_options = ToastOptions::new().description(summary);
            toast_api.success(title, toast_options);
        });
    };
    if !has_file {
        return ResolvePageView::NoFile;
    }
    if move_count == 0 && unresolved_count == 0 {
        return ResolvePageView::Clear;
    }
    let plan = plan_option.expect("plan present when a file is loaded");
    let selected_slug = selected_move_category.read().clone();
    let active = plan.active_section(selected_slug.as_deref());
    let active_category = active.map(|section| section.category);
    let mut breadcrumb_list: Vec<BreadcrumbProps> = Vec::with_capacity(plan.sections.len());
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
        let breadcrumb = BreadcrumbProps {
            title,
            count,
            data_breadcrumb,
            active: is_active,
            onclick,
        };
        breadcrumb_list.push(breadcrumb);
    }
    let breadcrumbs = BreadcrumbsProps {
        breadcrumbs: breadcrumb_list,
    };
    let section = active.map(|section| {
        let rows: Vec<MoveRowProps> = section
            .moves
            .iter()
            .map(|move_view| MoveRowProps {
                move_view: move_view.clone(),
                view_navigation,
                carriers_dialog,
            })
            .collect();
        PlanBodySection {
            data_category: section.category.data_breadcrumb(),
            rows,
        }
    });
    let unresolved_rows: Vec<UnresolvedRowProps> = plan
        .unresolved
        .iter()
        .map(|unresolved_view| UnresolvedRowProps {
            unresolved_view: unresolved_view.clone(),
            carriers_dialog,
        })
        .collect();
    let body = PlanBodyProps {
        section,
        unresolved_rows,
    };
    let move_noun = if move_count == 1 { "move" } else { "moves" };
    let moves_text = format!("{move_count} {move_noun}");
    let apply_handler = EventHandler::new(handle_apply);
    let header = PlanHeaderProps {
        moves_text,
        unresolved_count,
        running: running_now,
        on_apply: apply_handler,
    };
    let carriers_dialog_host = CarriersDialogHostProps {
        carriers_dialog,
        view_navigation,
    };
    let presentation = ResolvePlanPresentation {
        move_count,
        unresolved_count,
        header,
        breadcrumbs,
        body,
        carriers_dialog_host,
    };
    ResolvePageView::Plan(Box::new(presentation))
}
