use dioxus::prelude::*;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");

const DECORATION_CLASS: &str = "h-[2.4rem] w-auto flex-none [filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))] max-[480px]:h-[1.6rem]";
const TITLE_CLASS: &str = "m-0 font-friz-quadrata uppercase tracking-[0.08em] text-[2.5rem] leading-none text-warcraft-gold [text-shadow:1px_1px_0_#000,0_0_18px_rgba(255,206,99,0.35)] max-[1099px]:text-[clamp(16px,5vw,22px)] max-[1099px]:tracking-[0.04em] max-[1099px]:whitespace-nowrap max-[1099px]:overflow-hidden max-[1099px]:text-ellipsis max-[1099px]:max-w-full";
const CLOSE_BUTTON_CLASS: &str = "close-button absolute right-4 top-1/2 -translate-y-1/2 w-10 h-10 flex items-center justify-center text-[1.5rem] font-friz-quadrata cursor-pointer transition-[color,text-shadow] duration-150 bg-transparent border-0 text-warcraft-text-secondary [text-shadow:1px_1px_0_#000] hover:text-warcraft-gold hover:[text-shadow:1px_1px_0_#000,0_0_12px_rgba(255,206,99,0.55)] focus:outline-none [body[data-kb-modality]_&]:focus:text-white [body[data-kb-modality]_&]:focus:[text-shadow:1px_1px_0_#000,0_0_16px_rgba(255,255,255,0.7)]";

#[component]
pub(crate) fn DialogHeader(title: String, on_close: EventHandler<()>) -> Element {
    let handle_close = move |_| on_close.call(());
    rsx! {
        header {
            class: "relative flex items-center justify-center gap-6 flex-none pt-[1.6rem] px-[4.5rem] pb-[1.4rem] [border-bottom:1px_solid_rgba(255,206,99,0.4)] [box-shadow:0_1px_0_rgba(0,0,0,0.7),0_2px_0_rgba(255,206,99,0.1)]",
            img {
                class: DECORATION_CLASS,
                src: HEADER_GOLD_DECORATION,
                alt: "",
                aria_hidden: "true",
            }
            h2 { class: TITLE_CLASS, "{title}" }
            img {
                class: "{DECORATION_CLASS} [transform:scaleX(-1)]",
                src: HEADER_GOLD_DECORATION,
                alt: "",
                aria_hidden: "true",
            }
            button {
                class: CLOSE_BUTTON_CLASS,
                r#type: "button",
                aria_label: "Close",
                onclick: handle_close,
                "\u{2715}"
            }
        }
    }
}
