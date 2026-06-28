use dioxus::prelude::*;
use warcraft_keybinds::ResolvedTemplate;

use crate::model::icons::IconUrl;

const TEMPLATE_CARD_GRID_LABEL: &str = "font-friz-quadrata text-[1.55rem] uppercase tracking-[0.1em] text-[rgba(255,206,99,0.75)] [text-shadow:1px_1px_0_#000] max-[1099px]:text-[10px] max-[1099px]:text-left";

const TEMPLATE_CARD_CELL_BASE: &str = "relative flex items-center justify-center bg-[rgba(8,18,35,0.6)] border border-[rgba(74,112,144,0.55)] rounded-[5px] overflow-hidden";
const TEMPLATE_CARD_CELL_FILLED: &str = "bg-[rgba(20,35,60,0.85)] border-[rgba(255,206,99,0.4)]";

const TEMPLATE_CELL_HOTKEY_BASE: &str = "absolute top-[3px] right-[3px] min-w-[1.4rem] h-[1.4rem] px-[0.3rem] py-0 flex items-center justify-center font-friz-quadrata text-[1.1rem] leading-none border rounded-[3px] [text-shadow:1px_1px_0_#000] pointer-events-none max-[1099px]:text-[9px] max-[1099px]:min-w-[12px] max-[1099px]:h-[12px] max-[1099px]:top-[1px] max-[1099px]:right-[1px] max-[1099px]:px-[2px] max-[1099px]:rounded-[2px]";
const TEMPLATE_CELL_HOTKEY_NORMAL: &str =
    "text-warcraft-gold bg-[rgba(0,0,0,0.88)] border-[rgba(255,206,99,0.65)]";
const TEMPLATE_CELL_HOTKEY_PASSIVE: &str = "text-[#b8bcc4] bg-[#1a1f29] border-[#4a5160]";

#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardGridProps {
    pub label: String,
    pub resolved: ResolvedTemplate,
    pub is_research: bool,
}

#[component]
pub fn TemplateCardGrid(props: TemplateCardGridProps) -> Element {
    let label = props.label;
    let resolved = props.resolved;
    let is_research = props.is_research;
    let tiles = if is_research {
        resolved.research_tiles()
    } else {
        resolved.command_tiles()
    };
    rsx! {
        div { class: "flex flex-col gap-[0.85rem] [flex:1_1_auto] max-[1099px]:[flex:1_1_0] max-[1099px]:min-w-0 max-[1099px]:w-auto max-[1099px]:items-stretch max-[1099px]:gap-[5px]",
            div { class: TEMPLATE_CARD_GRID_LABEL, {label} }
            div { class: "grid grid-cols-4 [grid-auto-rows:1fr] [aspect-ratio:4/3] gap-[0.55rem] w-full max-w-[26rem] max-[1099px]:max-w-full max-[1099px]:gap-[3px]", aria_hidden: "true",
                for tile in tiles.iter() {
                    {
                        let icon_source = tile
                            .icon()
                            .map(IconUrl::from_icon_path)
                            .map(|url| url.to_string());
                        let displayed_letter = tile.hotkey().display_label();
                        let cell_class = if tile.occupant().is_some() {
                            format!("{TEMPLATE_CARD_CELL_BASE} {TEMPLATE_CARD_CELL_FILLED}")
                        } else {
                            TEMPLATE_CARD_CELL_BASE.to_string()
                        };
                        let hotkey_class = if tile.is_passive() {
                            format!("{TEMPLATE_CELL_HOTKEY_BASE} {TEMPLATE_CELL_HOTKEY_PASSIVE}")
                        } else {
                            format!("{TEMPLATE_CELL_HOTKEY_BASE} {TEMPLATE_CELL_HOTKEY_NORMAL}")
                        };
                        rsx! {
                            span { class: cell_class,
                                if let Some(source) = icon_source {
                                    img {
                                        class: "w-full h-full object-cover",
                                        src: source,
                                        alt: "",
                                        draggable: "false",
                                        loading: "lazy",
                                    }
                                }
                                span { class: hotkey_class, {displayed_letter} }
                            }
                        }
                    }
                }
            }
        }
    }
}
