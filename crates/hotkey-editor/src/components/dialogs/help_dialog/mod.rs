use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::dialogs::dialog_header::DialogHeader;
use crate::components::shared::icons::{
    ICON_COG, ICON_COLLISIONS, ICON_DOWNLOAD, ICON_GRID, ICON_HELP, ICON_PREVIEW, ICON_REDO,
    ICON_RESOLVE, ICON_TEMPLATES, ICON_UNDO, ICON_UPLOAD,
};
use crate::services::customkeys::persistence::OnboardingPersistence;

const HELP_DIALOG_STYLES: Asset = asset!("/src/components/dialogs/help_dialog/help_dialog.css");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GlossaryEntry {
    term: &'static str,
    description: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct LegendEntry {
    icon: &'static str,
    label: &'static str,
    description: &'static str,
}

const CONFLICT_KINDS: [GlossaryEntry; 3] = [
    GlossaryEntry {
        term: "Cross unit collisions",
        description: "Two or more different units want to place an ability in the same grid cell. A popular ability can pull a whole crowd of units into one knot. This is a position conflict between units.",
    },
    GlossaryEntry {
        term: "Intra unit collisions",
        description: "One unit has two or more abilities competing for the same grid cell. This is a position conflict inside a single unit.",
    },
    GlossaryEntry {
        term: "Hotkey collisions",
        description: "Two or more abilities on the same unit are bound to the same hotkey letter.",
    },
];

const RESOLVER_WALKTHROUGH: [&str; 1] = [
    "The whole clash is modeled as a conflict graph. Every ability is a node, and two of them share an edge whenever a single unit carries both. The tangled clusters those edges form are the islands. Each island is solved on its own. Within one, an anchor is crowned at every contested button, the ability the most units carry, while the losers slide aside. A slide can land on another taken button and spark a fresh fight, so the moves cascade across the row. When a row runs out of room, the stranded ability spills onto another row or swaps with what sits there, and gap pulls tidy the holes left behind. The board is swept round after round until that island settles. Every island found is resolved the same way, so the whole grid lands in one go.",
];

const RESOLVER_PARTS: [GlossaryEntry; 3] = [
    GlossaryEntry {
        term: "Island",
        description: "A connected cluster of abilities that collide on one button. Two abilities are linked when a single unit carries both, and the links can chain from one ability to the next across several units.",
    },
    GlossaryEntry {
        term: "Anchor",
        description: "The ability that wins a contested button and stays put. The editor keeps the ability that the most units carry, because moving it would disturb the most cards, so the rarer ability gives way instead.",
    },
    GlossaryEntry {
        term: "Mover",
        description: "An ability that steps aside so the anchor can keep the button. It slides one button to the right.",
    },
];

const RESOLVER_MOVES: [GlossaryEntry; 4] = [
    GlossaryEntry {
        term: "Fights",
        description: "Two or more abilities want the same button. The most shared one stays, the others slide to the right.",
    },
    GlossaryEntry {
        term: "Gap pulls",
        description: "When a slide leaves an empty button with a filled one further along the same row, the editor pulls that ability back to close the gap and keep the row tidy.",
    },
    GlossaryEntry {
        term: "Spills",
        description: "If an ability still has nowhere to go in its own row, the editor rehomes it to a free button in another row.",
    },
    GlossaryEntry {
        term: "Swaps",
        description: "Two abilities swap positions in a single move.",
    },
];

const LEGEND_ENTRIES: [LegendEntry; 11] = [
    LegendEntry {
        icon: ICON_GRID,
        label: "Grid Layout",
        description: "Define a global grid and apply it to every unit.",
    },
    LegendEntry {
        icon: ICON_COLLISIONS,
        label: "Collisions",
        description: "Review the conflicts the editor found.",
    },
    LegendEntry {
        icon: ICON_TEMPLATES,
        label: "Templates",
        description: "Apply a prepared keybind set.",
    },
    LegendEntry {
        icon: ICON_UPLOAD,
        label: "Upload",
        description: "Import a CustomKeys.txt file from your computer.",
    },
    LegendEntry {
        icon: ICON_COG,
        label: "System Hotkeys",
        description: "Edit the system and menu hotkeys.",
    },
    LegendEntry {
        icon: ICON_RESOLVE,
        label: "Resolve",
        description: "Display and settle all positional conflicts at once.",
    },
    LegendEntry {
        icon: ICON_PREVIEW,
        label: "Preview",
        description: "See the text the editor will export.",
    },
    LegendEntry {
        icon: ICON_DOWNLOAD,
        label: "Export",
        description: "Download your CustomKeys.txt file.",
    },
    LegendEntry {
        icon: ICON_UNDO,
        label: "Undo",
        description: "Step backward through your changes.",
    },
    LegendEntry {
        icon: ICON_REDO,
        label: "Redo",
        description: "Step forward through your changes.",
    },
    LegendEntry {
        icon: ICON_HELP,
        label: "Help",
        description: "Reopen this guide at any time.",
    },
];

#[derive(Props, Clone, PartialEq)]
pub(crate) struct HelpDialogProps {
    pub(crate) help_open: Signal<bool>,
}

#[component]
pub(crate) fn HelpDialog(props: HelpDialogProps) -> Element {
    let mut help_open = props.help_open;
    let handle_open_change = move |is_open| help_open.set(is_open);
    let handle_close = move |_| help_open.set(false);
    let dismiss_for_good = move |_| {
        OnboardingPersistence::mark_seen();
        help_open.set(false);
    };
    rsx! {
        document::Stylesheet { href: HELP_DIALOG_STYLES }
        DialogRoot {
            class: "dialog-overlay",
            open: help_open(),
            on_open_change: handle_open_change,
            DialogContent { class: "dialog-shell wc3-dialog help-dialog".to_string(),
                DialogHeader {
                    title: "How to use this editor".to_string(),
                    on_close: handle_close,
                }
                div { class: "wc3-dialog-body help-dialog-body flex flex-col gap-[2.6rem] \
                        max-[1099px]:[flex:1_1_0] max-[1099px]:min-h-0 max-[1099px]:overflow-y-auto \
                        max-[1099px]:[-webkit-overflow-scrolling:touch] max-[1099px]:[overscroll-behavior:contain]",
                    div { class: "help-top-row flex flex-col gap-[2.6rem]",
                        div { class: "help-col-main flex flex-col gap-[2.6rem]",
                            section { class: "flex flex-col gap-[1.2rem]",
                                h3 { class: "help-section-title", "The workflow" }
                                div { class: "help-callout",
                                    ol { class: "help-workflow flex flex-col gap-[0.9rem] m-0 p-0",
                                        li { class: "help-workflow-step",
                                            "On your first visit the editor starts from the Warcraft III default keybinds."
                                        }
                                        li { class: "help-workflow-step",
                                            "When you come back it restores the progress you left, saved in your browser."
                                        }
                                        li { class: "help-workflow-step",
                                            "Apply a template "
                                            span {
                                                class: "help-inline-icon",
                                                aria_hidden: "true",
                                                dangerous_inner_html: ICON_TEMPLATES,
                                            }
                                            " to start from a known setup."
                                        }
                                        li { class: "help-workflow-step",
                                            "You might also just import "
                                            span {
                                                class: "help-inline-icon",
                                                aria_hidden: "true",
                                                dangerous_inner_html: ICON_UPLOAD,
                                            }
                                            " your own CustomKeys.txt file."
                                        }
                                        li { class: "help-workflow-step",
                                            "Open the collisions "
                                            span {
                                                class: "help-inline-icon",
                                                aria_hidden: "true",
                                                dangerous_inner_html: ICON_COLLISIONS,
                                            }
                                            " page to see every conflict the editor found."
                                        }
                                        li { class: "help-workflow-step",
                                            "Click the resolve "
                                            span {
                                                class: "help-inline-icon",
                                                aria_hidden: "true",
                                                dangerous_inner_html: ICON_RESOLVE,
                                            }
                                            " button in the toolbar to open the resolver page."
                                        }
                                        li { class: "help-workflow-step",
                                            "Press apply on the resolver page to settle every positional conflict at once."
                                        }
                                        li { class: "help-workflow-step",
                                            "Open the grid layout "
                                            span {
                                                class: "help-inline-icon",
                                                aria_hidden: "true",
                                                dangerous_inner_html: ICON_GRID,
                                            }
                                            " editor and set a hotkey for each button position."
                                        }
                                        li { class: "help-workflow-step",
                                            "Apply that grid to every unit to clear any remaining hotkey conflicts."
                                        }
                                        li { class: "help-workflow-step",
                                            "Select a unit and drag its abilities between cells to personalize the layout."
                                        }
                                        li { class: "help-workflow-step",
                                            "Edit menu and system keys from the system hotkeys "
                                            span {
                                                class: "help-inline-icon",
                                                aria_hidden: "true",
                                                dangerous_inner_html: ICON_COG,
                                            }
                                            " dialog."
                                        }
                                        li { class: "help-workflow-step",
                                            "Open the preview "
                                            span {
                                                class: "help-inline-icon",
                                                aria_hidden: "true",
                                                dangerous_inner_html: ICON_PREVIEW,
                                            }
                                            " to check the text the editor will export."
                                        }
                                        li { class: "help-workflow-step",
                                            "Export "
                                            span {
                                                class: "help-inline-icon",
                                                aria_hidden: "true",
                                                dangerous_inner_html: ICON_DOWNLOAD,
                                            }
                                            " your CustomKeys.txt file to where Warcraft III expects."
                                        }
                                        li { class: "help-workflow-step",
                                            "Enjoy your custom keybinds."
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "help-col-side flex flex-col gap-[2.6rem]",
                            section { class: "flex flex-col gap-[1.2rem]",
                                h3 { class: "help-section-title", "Button legend" }
                                ul { class: "help-legend flex flex-col gap-[0.9rem] m-0 p-0",
                                    for entry in LEGEND_ENTRIES.iter() {
                                        li { class: "help-legend-row flex items-center gap-[1.2rem]",
                                            span {
                                                class: "help-legend-icon inline-flex items-center justify-center \
                                                        shrink-0 w-[3rem] h-[3rem] [&_svg]:w-[2rem] [&_svg]:h-[2rem]",
                                                aria_hidden: "true",
                                                dangerous_inner_html: entry.icon,
                                            }
                                            span { class: "help-legend-text",
                                                span { class: "help-legend-label", "{entry.label}" }
                                                span { class: "help-legend-description", " {entry.description}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    section { class: "flex flex-col gap-[1.4rem]",
                        h3 { class: "help-section-title", "What the resolver is doing" }
                        div { class: "help-resolver-prose flex flex-col gap-[1.2rem]",
                            for paragraph in RESOLVER_WALKTHROUGH.iter() {
                                p { class: "help-body-text m-0", "{paragraph}" }
                            }
                        }
                        div { class: "help-glossary-columns flex flex-col gap-[2.6rem]",
                            div { class: "help-glossary-col flex flex-col gap-[1.4rem]",
                                for kind in CONFLICT_KINDS.iter() {
                                    div { class: "flex flex-col gap-[0.4rem]",
                                        p { class: "help-step-number m-0", "{kind.term}" }
                                        p { class: "help-body-text m-0", "{kind.description}" }
                                    }
                                }
                            }
                            div { class: "help-glossary-col flex flex-col gap-[1.4rem]",
                                for part in RESOLVER_PARTS.iter() {
                                    div { class: "flex flex-col gap-[0.4rem]",
                                        p { class: "help-step-number m-0", "{part.term}" }
                                        p { class: "help-body-text m-0", "{part.description}" }
                                    }
                                }
                            }
                            div { class: "help-glossary-col flex flex-col gap-[1.4rem]",
                                for resolver_move in RESOLVER_MOVES.iter() {
                                    div { class: "flex flex-col gap-[0.4rem]",
                                        p { class: "help-step-number m-0", "{resolver_move.term}" }
                                        p { class: "help-body-text m-0", "{resolver_move.description}" }
                                    }
                                }
                            }
                        }
                    }
                }
                footer { class: "flex items-center justify-end flex-none gap-4 pt-[1.4rem] px-[4.5rem] pb-[1.8rem] \
                        [border-top:1px_solid_rgba(255,206,99,0.4)] max-[1099px]:justify-center max-[480px]:px-6",
                    button {
                        class: "help-dismiss-button inline-flex items-center justify-center min-h-12 \
                                px-[1.8rem] py-[0.7rem] \
                                [background:linear-gradient(135deg,rgba(40,30,8,0.85)_0%,rgba(15,12,4,0.85)_100%)] \
                                border border-warcraft-gold rounded-[10px] text-warcraft-gold \
                                font-friz-quadrata text-[1.4rem] tracking-[0.08em] uppercase cursor-pointer \
                                [box-shadow:0_0_22px_rgba(255,206,99,0.22)] \
                                [transition:background_0.12s_ease,box-shadow_0.12s_ease] \
                                [@media(hover:hover)]:hover:[background:linear-gradient(135deg,rgba(255,206,99,0.22)_0%,rgba(60,45,14,0.95)_100%)] \
                                [@media(hover:hover)]:hover:[box-shadow:0_0_26px_rgba(255,206,99,0.55)] \
                                focus:outline-none \
                                focus-visible:border-white focus-visible:text-white \
                                focus-visible:[box-shadow:0_0_0_3px_#fff,0_0_18px_rgba(255,255,255,0.55)]",
                        r#type: "button",
                        onclick: dismiss_for_good,
                        "Got it, don't show this again"
                    }
                }
            }
        }
    }
}
