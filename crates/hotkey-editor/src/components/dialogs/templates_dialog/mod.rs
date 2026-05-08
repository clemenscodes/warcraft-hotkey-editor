use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use dioxus_primitives::toast::{ToastOptions, use_toast};
use warcraft_keybinds::{ColumnIndex, CustomKeys, RowIndex};

use crate::components::dialogs::dialog_header::DialogHeader;

use crate::model::grid::ResolvedTemplate;
use crate::model::grid::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS};
use crate::model::icons::IconUrl;
use crate::services::customkeys::upload_status::UploadStatus;
use warcraft_database::ObjectLookup;

const TEMPLATE_CARD: &str = "flex flex-col gap-[2.25rem] py-[2.5rem] px-[2.75rem] \
    [background:linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)] \
    border border-[#6c5a1f] rounded-[14px] text-[#c0c8da] cursor-pointer text-left \
    [transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease] \
    hover:border-warcraft-gold hover:text-warcraft-gold \
    hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)] \
    hover:[box-shadow:0_0_14px_rgba(255,206,99,0.35)] \
    focus:outline-none \
    [body[data-kb-modality]_&]:focus:outline-none \
    [body[data-kb-modality]_&]:focus:border-white \
    [body[data-kb-modality]_&]:focus:text-white \
    [body[data-kb-modality]_&]:focus:[box-shadow:0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] \
    max-[1099px]:py-[12px] max-[1099px]:px-[14px] max-[1099px]:gap-[10px] max-[1099px]:rounded-[10px]";

const TEMPLATE_CARD_NAME: &str = "m-0 font-friz-quadrata text-[2.75rem] uppercase tracking-[0.08em] text-inherit [text-shadow:1px_1px_0_#000] max-[1099px]:text-[clamp(17px,5vw,24px)] max-[1099px]:tracking-[0.06em] max-[1099px]:text-warcraft-gold";

const TEMPLATE_CARD_DESCRIPTION: &str = "m-0 text-[1.6rem] leading-[1.45] opacity-80 max-[1099px]:text-[13px] max-[1099px]:leading-[1.35] max-[1099px]:text-[#c0c8da] max-[1099px]:opacity-90";

const TEMPLATE_CARD_GRID_LABEL: &str = "font-friz-quadrata text-[1.55rem] uppercase tracking-[0.1em] text-[rgba(255,206,99,0.75)] [text-shadow:1px_1px_0_#000] max-[1099px]:text-[10px] max-[1099px]:text-left";

const TEMPLATE_CARD_CELL_BASE: &str = "relative flex items-center justify-center bg-[rgba(8,18,35,0.6)] border border-[rgba(74,112,144,0.55)] rounded-[5px] overflow-hidden";
const TEMPLATE_CARD_CELL_FILLED: &str = "bg-[rgba(20,35,60,0.85)] border-[rgba(255,206,99,0.4)]";

const TEMPLATE_CELL_HOTKEY_BASE: &str = "absolute top-[3px] right-[3px] min-w-[1.4rem] h-[1.4rem] px-[0.3rem] py-0 flex items-center justify-center font-friz-quadrata text-[1.1rem] leading-none border rounded-[3px] [text-shadow:1px_1px_0_#000] pointer-events-none max-[1099px]:text-[9px] max-[1099px]:min-w-[12px] max-[1099px]:h-[12px] max-[1099px]:top-[1px] max-[1099px]:right-[1px] max-[1099px]:px-[2px] max-[1099px]:rounded-[2px]";
const TEMPLATE_CELL_HOTKEY_NORMAL: &str =
    "text-warcraft-gold bg-[rgba(0,0,0,0.88)] border-[rgba(255,206,99,0.65)]";
const TEMPLATE_CELL_HOTKEY_PASSIVE: &str = "text-[#b8bcc4] bg-[#1a1f29] border-[#4a5160]";

#[component]
pub(crate) fn TemplatesDialog(
    mut loaded_keys: Signal<Option<CustomKeys>>,
    mut upload_status: Signal<UploadStatus>,
    mut templates_dialog_open: Signal<bool>,
) -> Element {
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
    let handle_click = move |_| on_apply.call(());
    rsx! {
        button {
            class: TEMPLATE_CARD,
            r#type: "button",
            onclick: handle_click,
            div { class: "flex flex-col gap-[0.5rem] max-[1099px]:gap-[4px]",
                h3 { class: TEMPLATE_CARD_NAME, "{template_name}" }
                p { class: TEMPLATE_CARD_DESCRIPTION, "{template_description}" }
            }
            div { class: "flex flex-row flex-wrap gap-[2rem] items-start max-[1099px]:flex-nowrap max-[1099px]:gap-[8px]",
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
        div { class: "flex flex-col gap-[0.85rem] [flex:1_1_auto] max-[1099px]:[flex:1_1_0] max-[1099px]:min-w-0 max-[1099px]:w-auto max-[1099px]:items-stretch max-[1099px]:gap-[5px]",
            div { class: TEMPLATE_CARD_GRID_LABEL, "{label}" }
            div { class: "grid grid-cols-4 [grid-auto-rows:1fr] [aspect-ratio:4/3] gap-[0.55rem] w-full max-w-[26rem] max-[1099px]:max-w-full max-[1099px]:gap-[3px]", aria_hidden: "true",
                for row in 0..COMMAND_GRID_ROWS {
                    for column in 0..COMMAND_GRID_COLUMNS {
                        {
                            let cell_option = if is_research {
                                resolved.research_menu_cell(column, row)
                            } else {
                                resolved.command_card_cell(column, row)
                            };
                            let icon_source = cell_option.and_then(|cell| cell.icon_path().map(IconUrl::from_icon_path));
                            let is_passive_command_cell = !is_research
                                && cell_option
                                    .map(|cell| ObjectLookup::is_passive_ability(cell.object_id().value()))
                                    .unwrap_or(false);
                            let binding_letter = cell_option.and_then(|cell| {
                                let token = if is_research {
                                    cell.binding_research_hotkey()
                                        .or_else(|| cell.binding_hotkey())
                                } else {
                                    cell.binding_hotkey()
                                };
                                token.map(|value| value.display_label())
                            });
                            let derived_letter = cell_option.and_then(|_| {
                                let col = ColumnIndex::try_from(column).ok()?;
                                let row_idx = RowIndex::try_from(row).ok()?;
                                resolved.grid().letter_at(col, row_idx).map(|character| character.to_string())
                            });
                            let displayed_letter = binding_letter.or(derived_letter);
                            let cell_class = if cell_option.is_some() {
                                format!("{TEMPLATE_CARD_CELL_BASE} {TEMPLATE_CARD_CELL_FILLED}")
                            } else {
                                TEMPLATE_CARD_CELL_BASE.to_string()
                            };
                            let hotkey_class = if is_passive_command_cell {
                                format!("{TEMPLATE_CELL_HOTKEY_BASE} {TEMPLATE_CELL_HOTKEY_PASSIVE}")
                            } else {
                                format!("{TEMPLATE_CELL_HOTKEY_BASE} {TEMPLATE_CELL_HOTKEY_NORMAL}")
                            };
                            rsx! {
                                span { class: cell_class,
                                    if let Some(source) = icon_source {
                                        img {
                                            class: "w-full h-full object-cover",
                                            src: "{source}",
                                            alt: "",
                                            draggable: "false",
                                            loading: "lazy",
                                        }
                                    }
                                    if let Some(letter) = displayed_letter {
                                        span { class: hotkey_class, "{letter}" }
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
