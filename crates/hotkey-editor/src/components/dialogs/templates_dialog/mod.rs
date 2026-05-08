mod template_card;
mod template_card_grid;

use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use dioxus_primitives::toast::{ToastOptions, use_toast};
use warcraft_keybinds::CustomKeys;

use crate::components::dialogs::dialog_header::DialogHeader;

use crate::model::grid::ResolvedTemplate;
use crate::services::customkeys::upload_status::UploadStatus;

use template_card::TemplateCard;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct TemplatesDialogProps {
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) upload_status: Signal<UploadStatus>,
    pub(crate) templates_dialog_open: Signal<bool>,
}

#[component]
pub(crate) fn TemplatesDialog(props: TemplatesDialogProps) -> Element {
    let mut loaded_keys = props.loaded_keys;
    let mut upload_status = props.upload_status;
    let mut templates_dialog_open = props.templates_dialog_open;
    let toast_api = use_toast();
    let resolved_templates = use_hook(ResolvedTemplate::resolve_all);

    let handle_open_change = move |is_open| templates_dialog_open.set(is_open);
    let handle_close = move |_| templates_dialog_open.set(false);
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: templates_dialog_open(),
            on_open_change: handle_open_change,
            DialogContent { class: "dialog-shell wc3-dialog templates-dialog-shell".to_string(),
                DialogHeader {
                    title: "Layout Templates".to_string(),
                    on_close: handle_close,
                }
                div { class: "wc3-dialog-body flex flex-col items-stretch gap-[2.5rem]",
                    div { class: "grid grid-cols-2 gap-[2.25rem] w-full max-[1099px]:grid-cols-1 max-[1099px]:gap-[10px]",
                        for resolved in resolved_templates.iter() {
                            {
                                let template_name: &'static str = resolved.template().name();
                                let template_description: &'static str = resolved.template().description();
                                let template_content: &'static str = resolved.template().content();
                                let template_resolved = resolved.clone();
                                let apply_template = move |_| {
                                    let parsed_template = CustomKeys::from(template_content);
                                    let binding_count = parsed_template.bindings_in_order().count();
                                    let command_count = parsed_template.commands_in_order().count();
                                    let mut baseline = CustomKeys::from(warcraft_keybinds::DEFAULT_CUSTOM_KEYS);
                                    baseline.extend(parsed_template);
                                    let normalized = baseline.normalize();
                                    loaded_keys.set(Some(normalized));
                                    upload_status.set(UploadStatus::Loaded {
                                        binding_count,
                                        command_count,
                                    });
                                    let summary = format!("{binding_count} ability bindings, {command_count} command bindings loaded");
                                    toast_api.success(
                                        format!("{template_name} applied"),
                                        ToastOptions::new().description(summary),
                                    );
                                    templates_dialog_open.set(false);
                                };
                                rsx! {
                                    TemplateCard {
                                        key: "{template_name}",
                                        template_name: template_name.to_string(),
                                        template_description: template_description.to_string(),
                                        template_content,
                                        template_resolved,
                                        on_apply: apply_template,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
