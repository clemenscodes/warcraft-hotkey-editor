use dioxus::prelude::*;

use crate::model::grid::ResolvedTemplate;

use super::template_card_grid::TemplateCardGrid;

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

#[derive(Props, Clone, PartialEq)]
pub(super) struct TemplateCardProps {
    pub(super) template_name: String,
    pub(super) template_description: String,
    pub(super) template_content: &'static str,
    pub(super) template_resolved: ResolvedTemplate,
    pub(super) on_apply: EventHandler<()>,
}

#[component]
pub(super) fn TemplateCard(props: TemplateCardProps) -> Element {
    let template_name = props.template_name;
    let template_description = props.template_description;
    let template_content = props.template_content;
    let template_resolved = props.template_resolved;
    let on_apply = props.on_apply;
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
                h3 { class: TEMPLATE_CARD_NAME, {template_name} }
                p { class: TEMPLATE_CARD_DESCRIPTION, {template_description} }
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
