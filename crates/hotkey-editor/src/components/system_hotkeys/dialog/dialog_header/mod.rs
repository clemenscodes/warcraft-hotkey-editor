use dioxus::prelude::*;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");

#[derive(Props, Clone, PartialEq)]
pub(super) struct SystemHotkeysHeaderProps {
    pub(super) on_close: EventHandler<()>,
}

#[component]
pub(super) fn SystemHotkeysHeader(props: SystemHotkeysHeaderProps) -> Element {
    let on_close = props.on_close;
    let handle_close = move |_| on_close.call(());
    rsx! {
        header {
            class: "relative flex items-center justify-center gap-6 flex-none pt-[1.6rem] px-[4.5rem] pb-[1.4rem] [border-bottom:1px_solid_rgba(255,206,99,0.4)] [box-shadow:0_1px_0_rgba(0,0,0,0.7),0_2px_0_rgba(255,206,99,0.1)] max-[1099px]:[padding:0.85rem_3rem_0.7rem] max-[1099px]:gap-[0.5rem] max-[1099px]:sticky max-[1099px]:top-0 max-[1099px]:z-[5] max-[1099px]:bg-[linear-gradient(135deg,rgba(12,25,50,0.98)_0%,rgba(6,12,28,0.98)_100%)]",
            img {
                class: "h-[2.4rem] w-auto flex-none [filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))]",
                src: HEADER_GOLD_DECORATION,
                alt: "",
            }
            h2 {
                class: "m-0 font-friz-quadrata uppercase tracking-[0.08em] text-[2.5rem] leading-none text-warcraft-gold [text-shadow:1px_1px_0_#000,0_0_18px_rgba(255,206,99,0.35)] max-[1099px]:text-[clamp(16px,5vw,22px)] max-[1099px]:tracking-[0.04em] max-[1099px]:whitespace-nowrap max-[1099px]:overflow-hidden max-[1099px]:text-ellipsis max-[1099px]:max-w-full",
                "System Hotkeys"
            }
            img {
                class: "h-[2.4rem] w-auto flex-none [filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))] [transform:scaleX(-1)]",
                src: HEADER_GOLD_DECORATION,
                alt: "",
            }
            button {
                class: "close-button absolute right-4 top-1/2 -translate-y-1/2 w-10 h-10 flex items-center justify-center text-[1.5rem] font-friz-quadrata cursor-pointer transition-[color,text-shadow] duration-150 bg-transparent border-0 text-warcraft-text-secondary [text-shadow:1px_1px_0_#000] hover:text-warcraft-gold hover:[text-shadow:1px_1px_0_#000,0_0_12px_rgba(255,206,99,0.55)] focus:outline-none [body[data-kb-modality]_&]:focus:text-white [body[data-kb-modality]_&]:focus:[text-shadow:1px_1px_0_#000,0_0_16px_rgba(255,255,255,0.7)] max-[1099px]:right-[0.5rem] max-[1099px]:w-[44px] max-[1099px]:h-[44px] max-[1099px]:text-[1.25rem]",
                r#type: "button",
                aria_label: "Close",
                onclick: handle_close,
                "\u{2715}"
            }
        }
    }
}
