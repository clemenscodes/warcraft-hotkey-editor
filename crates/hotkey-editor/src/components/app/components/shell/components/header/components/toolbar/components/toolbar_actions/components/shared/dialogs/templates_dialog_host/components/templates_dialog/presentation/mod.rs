use super::components::templates_dialog_panel::components::templates_dialog_body::components::template_gallery::components::template_card::TemplateCardView;
use dioxus::prelude::*;

/// The templates dialog's own shell, shaped from its view: the open value driving
/// the backdrop, the change handler that writes the open signal, and the panel's own
/// domain values — its header title, the close handler, and the resolved card views.
pub(super) struct TemplatesDialogShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
    pub(super) on_close: EventHandler<()>,
    pub(super) cards: Vec<TemplateCardView>,
}

impl From<&TemplatesDialogView> for TemplatesDialogShell {
    fn from(view: &TemplatesDialogView) -> Self {
        let mut open_signal = view.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = view.open;
        let title = String::from("Layout Templates");
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let cards = view.cards.clone();
        Self {
            open,
            on_open_change,
            title,
            on_close,
            cards,
        }
    }
}
use super::model::TemplatesDialogModel;
use crate::components::app::components::shell::components::toasts::ToastOptions;
use crate::components::app::components::shell::components::toasts::use_toast;
use crate::services::customkeys::upload_status::UploadStatus;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::ResolvedTemplate;

/// The templates dialog's shaped view: the open signal driving the shell and the
/// resolved template card views, each with its apply handler.
pub(super) struct TemplatesDialogView {
    pub(super) open: Signal<bool>,
    pub(super) cards: Vec<TemplateCardView>,
}

/// Composes the templates dialog's cards. Resolves every bundled template and,
/// for each, builds the apply handler that overwrites the loaded keys: parse the
/// template, extend the default baseline, normalize, write the signals, toast,
/// and close. All of that domain work lives here, never in the markup.
pub(super) fn use_templates_dialog(props: &TemplatesDialogModel) -> TemplatesDialogView {
    let open = props.open;
    let mut loaded_keys = props.loaded_keys;
    let mut upload_status = props.upload_status;
    let mut dialog_open = props.open;
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
            TemplateCardView {
                name,
                description,
                resolved: resolved_template,
                on_apply,
            }
        })
        .collect();
    TemplatesDialogView { open, cards }
}

impl ddd::Presentation for TemplatesDialogView {
    type Model = TemplatesDialogModel;
}
