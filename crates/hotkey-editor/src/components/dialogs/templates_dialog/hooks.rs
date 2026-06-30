use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use warcraft_keybinds::{CustomKeys, DEFAULT_CUSTOM_KEYS, ResolvedTemplate};

use crate::services::customkeys::upload_status::UploadStatus;

use super::components::template_gallery::components::template_card::TemplateCardProps;
use super::props::TemplatesDialogProps;

/// Composes the templates dialog's cards. Resolves every bundled template and,
/// for each, builds the apply handler that overwrites the loaded keys: parse the
/// template, extend the default baseline, normalize, write the signals, toast,
/// and close. All of that domain work lives here, never in the markup.
pub(super) fn use_templates_dialog(props: &TemplatesDialogProps) -> Vec<TemplateCardProps> {
    let mut loaded_keys = props.loaded_keys;
    let mut upload_status = props.upload_status;
    let mut templates_dialog_open = props.templates_dialog_open;
    let toast_api = use_toast();
    let resolved_templates = use_hook(ResolvedTemplate::resolve_all);
    resolved_templates
        .iter()
        .map(|resolved| {
            let name = resolved.name().to_string();
            let description = resolved.description().to_string();
            let template_content = resolved.content();
            let toast_name = name.clone();
            let resolved_template = resolved.clone();
            let on_apply = EventHandler::new(move |()| {
                let parsed_template = CustomKeys::from(template_content);
                let binding_count = parsed_template.bindings_in_order().count();
                let command_count = parsed_template.commands_in_order().count();
                let mut baseline = CustomKeys::from(DEFAULT_CUSTOM_KEYS);
                baseline.extend(parsed_template);
                let normalized = baseline.normalize();
                loaded_keys.set(Some(normalized));
                let status = UploadStatus::Loaded {
                    binding_count,
                    command_count,
                };
                upload_status.set(status);
                let summary = format!(
                    "{binding_count} ability bindings, {command_count} command bindings loaded"
                );
                let options = ToastOptions::new().description(summary);
                let title = format!("{toast_name} applied");
                toast_api.success(title, options);
                templates_dialog_open.set(false);
            });
            TemplateCardProps {
                name,
                description,
                resolved: resolved_template,
                on_apply,
            }
        })
        .collect()
}
