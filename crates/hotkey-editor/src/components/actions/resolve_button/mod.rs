use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use gloo_timers::future::TimeoutFuture;
use warcraft_keybinds::CustomKeys;

use crate::components::dialogs::resolve_info_dialog::ResolveInfoDialog;
use crate::components::shared::icons::ICON_RESOLVE;
use crate::components::shell::header::{TOOLBAR_BTN_CLASS, TOOLBAR_ICON_CLASS};

#[derive(Props, Clone, PartialEq)]
pub(crate) struct ResolveButtonProps {
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
}

#[component]
pub(crate) fn ResolveButton(props: ResolveButtonProps) -> Element {
    let mut loaded_keys = props.loaded_keys;
    let has_loaded_file = loaded_keys.read().is_some();
    let mut info_open = use_signal(|| false);
    let mut is_running = use_signal(|| false);
    let toast_api = use_toast();

    let open_info = move |_| {
        if *is_running.read() {
            return;
        }
        info_open.set(true);
    };

    let handle_apply = move |_| {
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
            // Yield once so the dialog can paint the spinner before the
            // CPU-bound cascade blocks the main thread.  The cascade is
            // synchronous Rust; without this hand-off it would freeze the
            // UI for the duration of the run.
            TimeoutFuture::new(0).await;
            let mut working_copy = working_copy;
            let plan = working_copy.resolve_conflicts();
            let move_count = plan.move_count();
            let unresolved_count = plan.unresolved_count();
            let normalized = working_copy.normalize();
            loaded_keys.set(Some(normalized));
            is_running.set(false);
            info_open.set(false);
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

    rsx! {
        div { class: "contents",
            button {
                class: TOOLBAR_BTN_CLASS,
                r#type: "button",
                aria_label: "Resolve conflicts",
                aria_haspopup: "dialog",
                aria_expanded: "{info_open()}",
                disabled: !has_loaded_file,
                onclick: open_info,
                span {
                    class: TOOLBAR_ICON_CLASS,
                    aria_hidden: "true",
                    dangerous_inner_html: ICON_RESOLVE,
                }
            }
            ResolveInfoDialog {
                open: info_open,
                is_running,
                on_apply: handle_apply,
            }
        }
    }
}
