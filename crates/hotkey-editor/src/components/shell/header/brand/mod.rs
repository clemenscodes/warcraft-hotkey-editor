use dioxus::prelude::*;

use crate::services::navigation::app_view::AppView;
use crate::services::navigation::view_navigation::ViewNavigationContext;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");

#[derive(Props, Clone, PartialEq)]
pub(crate) struct HeaderBrandProps {
    pub(crate) navigation: ViewNavigationContext,
}

#[component]
pub(crate) fn HeaderBrand(props: HeaderBrandProps) -> Element {
    let navigation = props.navigation;
    let go_home = move |_| navigation.apply(AppView::Editor);
    rsx! {
        button {
            class: "flex flex-row items-center justify-start \
                    [gap:calc(1rem_*_var(--hdr-scale))] min-w-0 flex-1 \
                    bg-transparent border-0 p-0 cursor-pointer text-left \
                    [transition:filter_0.12s_ease,text-shadow_0.12s_ease] \
                    [@media(hover:hover)]:hover:[filter:brightness(1.15)] \
                    focus:outline-none focus-visible:[outline:2px_solid_#fff] focus-visible:[outline-offset:2px] \
                    max-[1099px]:gap-2 max-[1099px]:[flex:1_1_auto] \
                    min-[1500px]:[flex:unset]",
            r#type: "button",
            "aria-label": "Warcraft III Hotkey Editor — return to editor",
            "data-action": "view-editor",
            onclick: go_home,
            img {
                class: "[height:calc(2rem_*_var(--hdr-scale))] w-auto flex-none \
                        [filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))] \
                        hidden min-[1500px]:block",
                src: HEADER_GOLD_DECORATION,
                alt: "",
                aria_hidden: "true",
            }
            h1 {
                class: "m-0 font-friz-quadrata font-normal \
                        [font-size:calc(3.2rem_*_var(--hdr-scale))] \
                        leading-[1.1] tracking-[0.04em] text-warcraft-gold \
                        whitespace-normal break-words text-left \
                        [text-shadow:1px_1px_0_rgba(0,0,0,0.92),0_0_14px_rgba(255,206,99,0.18)] \
                        max-[1099px]:[font-size:clamp(15px,4.5vw,22px)] \
                        max-[1099px]:leading-[1.15] max-[1099px]:[flex:1_1_auto] \
                        max-[1099px]:min-w-0 max-[1099px]:overflow-hidden \
                        max-[1099px]:text-ellipsis max-[1099px]:whitespace-nowrap",
                "Warcraft III Hotkey Editor"
            }
            img {
                class: "[height:calc(2rem_*_var(--hdr-scale))] w-auto flex-none \
                        [filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))] \
                        [transform:scaleX(-1)] \
                        hidden min-[1500px]:block",
                src: HEADER_GOLD_DECORATION,
                alt: "",
                aria_hidden: "true",
            }
        }
    }
}
