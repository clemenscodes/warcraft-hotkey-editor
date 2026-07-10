use super::components::plan_body::PlanBodyProps;
use super::components::plan_header::PlanHeaderProps;
use super::logic::{ActivePlanInputs, ActivePlanView, PlanCounts, PlanView};
use super::props::ResolvePageProps;
use crate::components::app::components::shell::components::shared::breadcrumbs::BreadcrumbsProps;
use crate::components::app::components::shell::components::toasts::{ToastOptions, use_toast};
use crate::services::customkeys::context::{use_custom_keys_service, use_loaded_keys};
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::{use_synced_route, use_view_navigation};
use crate::services::navigation::nav_snapshot::NavSnapshot;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use crate::services::resolve_selection::ResolveSelection;
use crate::services::resolve_selection::context::use_resolve_selection;
use ddd::Service;
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

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
/// body child props.
pub(super) struct ResolvePlanPresentation {
    pub header: PlanHeaderProps,
    pub breadcrumbs: BreadcrumbsProps,
    pub body: PlanBodyProps,
}

/// The Apply control's state: whether the cascade is currently running and the
/// handler that spawns it. Owns the `is_running` signal and routes the cascade
/// through the [`CustomKeysService`](crate::services::customkeys) commit boundary —
/// never mutating `CustomKeys` inline — then reports the result via a toast.
pub(super) struct ApplyPlan {
    pub(super) running: bool,
    pub(super) on_apply: EventHandler<MouseEvent>,
}

/// Reconcile the live route into the shell's signals (the read side of the URL
/// contract): announce the Resolve view, sync the selected move-category from the
/// `?entry=` parameter, and push the matching nav snapshot. Reactive on `entry`.
fn use_route_reconcile(
    entry: Option<String>,
    view_navigation: ViewNavigationContext,
    resolve_selection: ResolveSelection,
    mut synced_route: Signal<NavSnapshot>,
) {
    use_effect(use_reactive!(|entry| {
        view_navigation.restore_view(AppView::Resolve);
        let mut selected = resolve_selection.selected_move_category();
        if *selected.peek() != entry {
            selected.set(entry.clone());
        }
        let snapshot = NavSnapshot::Resolve {
            entry: entry.clone(),
        };
        synced_route.set(snapshot);
    }));
}

/// Owns the running flag and builds the Apply handler. The handler yields a tick so
/// the running state paints, runs the cascade through the service commit boundary
/// (which normalizes and writes through to storage), and toasts the outcome.
fn use_apply_plan() -> ApplyPlan {
    let loaded_keys = use_loaded_keys();
    let custom_keys_service = use_custom_keys_service();
    let toast_api = use_toast();
    let mut is_running = use_signal(|| false);
    let running = *is_running.read();
    let on_apply = EventHandler::new(move |_event: MouseEvent| {
        if *is_running.read() {
            return;
        }
        let read_guard = loaded_keys.peek();
        if read_guard.as_ref().is_none() {
            return;
        }
        drop(read_guard);
        is_running.set(true);
        spawn(async move {
            TimeoutFuture::new(0).await;
            let plan = custom_keys_service.commit(|keys| keys.resolve_conflicts());
            let move_count = plan.move_count();
            let unresolved_count = plan.unresolved_count();
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
    });
    ApplyPlan { running, on_apply }
}

/// Computes the cascade preview (memoised on the loaded keys), reconciles the route,
/// wires the Apply handler, and shapes the active section, breadcrumbs, and header —
/// returning the state's data for the body to render.
pub(super) fn use_resolve_page(props: &ResolvePageProps) -> ResolvePageView {
    let view_navigation = use_view_navigation();
    let resolve_selection = use_resolve_selection();
    let custom_keys_service = use_custom_keys_service();
    let loaded_keys = use_loaded_keys();
    let selected_move_category = resolve_selection.selected_move_category();
    let synced_route = use_synced_route();
    let entry = props.entry.clone().filter(|value| !value.is_empty());
    use_route_reconcile(entry, view_navigation, resolve_selection, synced_route);
    let plan_memo = use_memo(move || {
        if loaded_keys.read().is_none() {
            return None;
        }
        let plan = custom_keys_service.resolve_preview();
        let plan_view = PlanView::build(&plan);
        Some(plan_view)
    });
    let apply = use_apply_plan();
    let has_file = loaded_keys.read().is_some();
    let plan_option = plan_memo();
    let counts = PlanCounts::from(plan_option.as_ref());
    if !has_file {
        return ResolvePageView::NoFile;
    }
    if counts.move_count == 0 && counts.unresolved_count == 0 {
        return ResolvePageView::Clear;
    }
    let plan = plan_option.expect("plan present when a file is loaded");
    let selected_slug = selected_move_category.read().clone();
    let active_inputs = ActivePlanInputs {
        plan: &plan,
        selected_slug: selected_slug.as_deref(),
        selection: selected_move_category,
    };
    let active = ActivePlanView::from(active_inputs);
    let move_count = counts.move_count;
    let move_noun = if move_count == 1 { "move" } else { "moves" };
    let moves_text = format!("{move_count} {move_noun}");
    let header = PlanHeaderProps {
        moves_text,
        unresolved_count: counts.unresolved_count,
        running: apply.running,
        on_apply: apply.on_apply,
    };
    let presentation = ResolvePlanPresentation {
        header,
        breadcrumbs: active.breadcrumbs,
        body: active.body,
    };
    ResolvePageView::Plan(Box::new(presentation))
}
