use super::components::templates_dialog_body::components::template_gallery::components::template_card::TemplateCardView;
use crate::components::app::components::shell::components::toasts::ToastOptions;
use crate::components::app::components::shell::components::toasts::use_toast;
use crate::services::customkeys::context::{use_custom_keys_service, use_upload_status};
use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::overlay_state::context::use_overlay_state;
use dioxus::prelude::*;
use warcraft_keybinds::ResolvedTemplate;

/// The templates dialog host's shaped wiring: whether the templates browser is open, the
/// change handler mirroring the headless dialog's own close (escape, outside click) back
/// to the shared signal, and the resolved template card views the body lays out — each with
/// its apply handler that overwrites the loaded document. Body-scroll lock is owned once by
/// `WarcraftDialog`, so this host only flips the shared signal.
pub(super) struct TemplatesDialogHostModel {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) cards: Vec<TemplateCardView>,
}

/// Reads context and shapes the templates dialog: the shared open signal the toolbar
/// buttons flip, the change handler mirroring the headless dialog's close back to that
/// signal, and the resolved cards. Each card's apply handler imports the bundled template
/// through the sanctioned `CustomKeysService::import_overlay` command (which overlays,
/// normalizes, and writes through to storage), sets the upload-status signal, toasts, and
/// closes the browser. All that domain work lives here, never in the markup.
pub(super) fn use_templates_dialog_host() -> TemplatesDialogHostModel {
    let custom_keys_service = use_custom_keys_service();
    let mut upload_status = use_upload_status();
    let overlay = use_overlay_state();
    let dialog_open = overlay.templates_dialog_open();
    let open = *dialog_open.read();
    let mut change_open = dialog_open;
    let on_open_change = Callback::new(move |is_open| change_open.set(is_open));
    let toast_api = use_toast();
    let resolved_templates = use_hook(ResolvedTemplate::resolve_all);
    let cards: Vec<TemplateCardView> = resolved_templates
        .iter()
        .map(|resolved| {
            let name = resolved.name().to_string();
            let description = resolved.description().to_string();
            let template_content = resolved.content();
            let toast_name = name.clone();
            let resolved_template = resolved.clone();
            let mut close_open = dialog_open;
            let on_apply = EventHandler::new(move |()| {
                let outcome = custom_keys_service.import_overlay(template_content);
                let binding_count = outcome.binding_count();
                let command_count = outcome.command_count();
                let status = UploadStatus::Loaded {
                    binding_count,
                    command_count,
                };
                upload_status.set(status);
                let summary = format!(
                    "{binding_count} ability bindings, {command_count} command bindings loaded",
                );
                let options = ToastOptions::new().description(summary);
                let title = format!("{toast_name} applied");
                toast_api.success(title, options);
                close_open.set(false);
            });
            TemplateCardView {
                name,
                description,
                resolved: resolved_template,
                on_apply,
            }
        })
        .collect();
    TemplatesDialogHostModel {
        open,
        on_open_change,
        cards,
    }
}
