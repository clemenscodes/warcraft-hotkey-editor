use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use dioxus_primitives::toast::{ToastOptions, use_toast};
use warcraft_keybinds::CustomKeysFile;

use crate::components::dialog_header::DialogHeader;
use crate::customkeys::baseline::BASELINE_CUSTOM_KEYS;
use crate::customkeys::upload_overlay::UploadOverlay;
use crate::customkeys::upload_status::UploadStatus;
use crate::domain::grid_layout::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridLayout};
use crate::domain::grid_templates::ResolvedTemplate;
use crate::domain::object_lookup::ObjectLookup;
use crate::domain::positions::Positions;

#[component]
pub(crate) fn TemplatesDialog(
    mut loaded_keys: Signal<Option<CustomKeysFile>>,
    mut upload_status: Signal<UploadStatus>,
    mut grid_layout: Signal<GridLayout>,
    mut templates_dialog_open: Signal<bool>,
) -> Element {
    let toast_api = use_toast();
    let resolved_templates = use_hook(ResolvedTemplate::resolve_all);

    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open: templates_dialog_open(),
            on_open_change: move |is_open| templates_dialog_open.set(is_open),
            DialogContent { class: "dialog-shell wc3-dialog templates-dialog-shell".to_string(),
                DialogHeader {
                    title: "Layout Templates".to_string(),
                    on_close: move |_| templates_dialog_open.set(false),
                }
                div { class: "wc3-dialog-body templates-dialog-body",
                    div { class: "templates-grid",
                        for resolved in resolved_templates.iter() {
                            {
                                let template_name: &'static str = resolved.template().name();
                                let template_description: &'static str = resolved.template().description();
                                let template_content: &'static str = resolved.template().content();
                                let template_resolved = resolved.clone();
                                rsx! {
                                    TemplateCard {
                                        key: "{template_name}",
                                        template_name: template_name.to_string(),
                                        template_description: template_description.to_string(),
                                        template_content,
                                        template_resolved,
                                        on_apply: move |_| {
                                            let mut parsed_template = CustomKeysFile::from(template_content);
                                            let binding_count = parsed_template.bindings_in_order().count();
                                            let command_count = parsed_template.commands_in_order().count();
                                            let import_layout = *grid_layout.read();
                                            Positions::fill_positions_from_hotkeys(&mut parsed_template, import_layout);
                                            let mut baseline = CustomKeysFile::from(BASELINE_CUSTOM_KEYS);
                                            UploadOverlay::apply(&mut baseline, &parsed_template);
                                            loaded_keys.set(Some(baseline));
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
                                        },
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

#[component]
fn TemplateCard(
    template_name: String,
    template_description: String,
    template_content: &'static str,
    template_resolved: ResolvedTemplate,
    on_apply: EventHandler<()>,
) -> Element {
    let _ = template_content;
    let command_card_resolved = template_resolved.clone();
    let research_menu_resolved = template_resolved.clone();
    rsx! {
        button {
            class: "template-card",
            r#type: "button",
            onclick: move |_| on_apply.call(()),
            div { class: "template-card-info",
                h3 { class: "template-card-name", "{template_name}" }
                p { class: "template-card-description", "{template_description}" }
            }
            div { class: "template-card-grids",
                TemplateCardGrid {
                    label: "Command card".to_string(),
                    resolved: command_card_resolved,
                    is_research: false,
                }
                TemplateCardGrid {
                    label: "Research menu".to_string(),
                    resolved: research_menu_resolved,
                    is_research: true,
                }
            }
        }
    }
}

#[component]
fn TemplateCardGrid(label: String, resolved: ResolvedTemplate, is_research: bool) -> Element {
    rsx! {
        div { class: "template-card-grid",
            div { class: "template-card-grid-label", "{label}" }
            div { class: "template-card-cells", aria_hidden: "true",
                for row in 0..COMMAND_GRID_ROWS {
                    for column in 0..COMMAND_GRID_COLUMNS {
                        {
                            let cell_option = if is_research {
                                resolved.research_menu_cell(column, row)
                            } else {
                                resolved.command_card_cell(column, row)
                            };
                            let icon_src = cell_option.and_then(|cell| cell.cloned_icon_src());
                            let is_passive_command_cell = !is_research
                                && cell_option
                                    .map(|cell| ObjectLookup::is_passive_ability(cell.object_id()))
                                    .unwrap_or(false);
                            let binding_letter = cell_option.and_then(|cell| {
                                if is_research {
                                    cell.binding_research_hotkey()
                                        .map(String::from)
                                        .or_else(|| cell.binding_hotkey().map(String::from))
                                } else {
                                    cell.binding_hotkey().map(String::from)
                                }
                            });
                            let derived_letter = cell_option.and_then(|_| {
                                resolved.grid().letter_at(column, row).map(|character| character.to_string())
                            });
                            let displayed_letter = binding_letter.or(derived_letter);
                            let mut cell_class = String::from("template-card-cell");
                            if cell_option.is_some() {
                                cell_class.push_str(" filled");
                            }
                            let hotkey_class = if is_passive_command_cell {
                                "template-card-cell-hotkey passive"
                            } else {
                                "template-card-cell-hotkey"
                            };
                            rsx! {
                                span { class: "{cell_class}",
                                    if let Some(source) = icon_src {
                                        img {
                                            class: "template-card-cell-icon",
                                            src: "{source}",
                                            alt: "",
                                            draggable: "false",
                                            loading: "lazy",
                                        }
                                    }
                                    if let Some(letter) = displayed_letter {
                                        span { class: "{hotkey_class}", "{letter}" }
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
