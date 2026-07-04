use super::components::template_gallery::TemplateGalleryProps;
use super::components::template_gallery::components::template_card::TemplateCardProps;
use super::props::TemplatesDialogProps;
use crate::components::app::components::shell::components::toasts::{ToastOptions, use_toast};
use crate::services::customkeys::upload_status::UploadStatus;
use dioxus::prelude::*;
use warcraft_keybinds::{CustomKeys, ResolvedTemplate};

/// The templates dialog's shaped view: the open signal driving the shell and the
/// gallery of resolved template cards, each with its apply handler.
pub(super) struct TemplatesDialogView {
    pub(super) open: Signal<bool>,
    pub(super) gallery: TemplateGalleryProps,
}

/// Composes the templates dialog's cards. Resolves every bundled template and,
/// for each, builds the apply handler that overwrites the loaded keys: parse the
/// template, extend the default baseline, normalize, write the signals, toast,
/// and close. All of that domain work lives here, never in the markup.
pub(super) fn use_templates_dialog(props: &TemplatesDialogProps) -> TemplatesDialogView {
    let open = props.open;
    let mut loaded_keys = props.loaded_keys;
    let mut upload_status = props.upload_status;
    let mut dialog_open = props.open;
    let toast_api = use_toast();
    let resolved_templates = use_hook(ResolvedTemplate::resolve_all);
    let cards: Vec<TemplateCardProps> = resolved_templates
        .iter()
        .map(|resolved| {
            let name = resolved.name().to_string();
            let description = resolved.description().to_string();
            let template_content = resolved.content();
            let toast_name = name.clone();
            let resolved_template = resolved.clone();
            let on_apply = EventHandler::new(move |()| {
                let outcome = CustomKeys::import_overlay(template_content);
                let binding_count = outcome.binding_count();
                let command_count = outcome.command_count();
                loaded_keys.set(Some(outcome.into_keys()));
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
                dialog_open.set(false);
            });
            TemplateCardProps {
                name,
                description,
                resolved: resolved_template,
                on_apply,
            }
        })
        .collect();
    let gallery = TemplateGalleryProps { cards };
    TemplatesDialogView { open, gallery }
}
