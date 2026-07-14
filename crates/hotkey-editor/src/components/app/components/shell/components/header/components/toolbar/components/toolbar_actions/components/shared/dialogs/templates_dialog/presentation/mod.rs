use super::components::templates_dialog_body::components::template_gallery::components::template_card::TemplateCardView;
use super::model::TemplatesDialogModel;
use crate::components::app::components::shell::components::toasts::ToastOptions;
use crate::components::app::components::shell::components::toasts::use_toast;
use crate::services::customkeys::context::{use_custom_keys_service, use_upload_status};
use crate::services::customkeys::upload_status::UploadStatus;
use dioxus::prelude::*;
use warcraft_keybinds::ResolvedTemplate;

/// The templates dialog's shaped wiring: whether the templates browser is open, the change
/// handler mirroring the headless dialog's own close (escape, outside click) back to the
/// trigger that owns the open signal, and the resolved template card views the body lays
/// out — each with its apply handler that overwrites the loaded document. Body-scroll lock
/// is owned once by `WarcraftDialog`, so this presentation only carries the trigger's handler.
pub(super) struct TemplatesDialogPresentation {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) cards: Vec<TemplateCardView>,
}

impl ddd::Presentation for TemplatesDialogPresentation {
    type Model = TemplatesDialogModel;
}

/// Reads context and shapes the templates dialog from the trigger's props: the open value and
/// change handler the trigger owns, plus the resolved cards. Each card's apply handler imports
/// the bundled template through the sanctioned `CustomKeysService::import_overlay` command
/// (which overlays, normalizes, and writes through to storage), sets the upload-status signal,
/// toasts, and closes the browser through the trigger's change handler. All that domain work
/// lives here, never in the markup.
pub(super) fn use_templates_dialog(props: &TemplatesDialogModel) -> TemplatesDialogPresentation {
    let custom_keys_service = use_custom_keys_service();
    let mut upload_status = use_upload_status();
    let open = props.open;
    let on_open_change = props.on_open_change;
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
            let on_close = props.on_open_change;
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
                on_close.call(false);
            });
            TemplateCardView {
                name,
                description,
                resolved: resolved_template,
                on_apply,
            }
        })
        .collect();
    TemplatesDialogPresentation {
        open,
        on_open_change,
        cards,
    }
}
