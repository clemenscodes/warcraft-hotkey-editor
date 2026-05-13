use dioxus::prelude::*;

use crate::services::navigation::app_view::CollisionKind;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct CollisionsPageProps {
    pub(crate) kind: CollisionKind,
}

/// Placeholder for the Collisions top-level page.  The real content is
/// implemented in issues #40 (position collisions) and #41 (hotkey
/// collisions); this stub establishes routing so deep-linking and the
/// header view switcher can land here.
#[component]
pub(crate) fn CollisionsPage(props: CollisionsPageProps) -> Element {
    let kind_label = match props.kind {
        CollisionKind::Positions => "Position Collisions",
        CollisionKind::Hotkeys => "Hotkey Collisions",
    };
    rsx! {
        section {
            class: "collisions-page flex flex-col items-center justify-center \
                    gap-[1.5rem] p-[2.5rem] text-warcraft-text-secondary text-center \
                    [flex:1_1_0] [min-height:0]",
            "data-collision-kind": match props.kind {
                CollisionKind::Positions => "positions",
                CollisionKind::Hotkeys => "hotkeys",
            },
            h2 {
                class: "font-friz-quadrata uppercase tracking-[0.14em] \
                        text-warcraft-gold text-[clamp(20px,2.6vw,32px)] m-0",
                "{kind_label}"
            }
            p {
                class: "max-w-[640px] m-0 leading-[1.6] text-[clamp(13px,1.4vw,18px)]",
                "This page lists every collision the cascade would resolve."
            }
        }
    }
}
