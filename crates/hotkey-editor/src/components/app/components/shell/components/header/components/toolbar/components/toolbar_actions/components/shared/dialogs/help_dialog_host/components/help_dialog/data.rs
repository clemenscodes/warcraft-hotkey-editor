use crate::components::app::components::shell::components::shared::icons::{
    ICON_COG, ICON_COLLISIONS, ICON_DOWNLOAD, ICON_GRID, ICON_HELP, ICON_PREVIEW, ICON_REDO,
    ICON_RESOLVE, ICON_TEMPLATES, ICON_UNDO, ICON_UPLOAD,
};

use super::components::help_dialog_panel::components::help_dialog_body::components::help_body::components::help_resolver_section::components::help_glossary_columns::components::help_glossary_column::components::help_glossary_entry::HelpGlossaryEntryProps;
use super::components::help_dialog_panel::components::help_dialog_body::components::help_body::components::help_top_row::components::help_legend_section::components::help_legend::components::help_legend_row::HelpLegendRowProps;

/// One piece of a workflow step: either a run of text or an inline toolbar glyph.
/// A step is a sequence of these, so the renderer is a pure loop and never bakes
/// copy or icons into markup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpSegment {
    Text { content: &'static str },
    Icon { svg: &'static str },
}

/// The whole onboarding guide's content, sourced here and threaded into the
/// renderer components as props. No UI component owns any of this.
#[derive(Clone, Copy, PartialEq)]
pub struct HelpContent {
    workflow: &'static [&'static [HelpSegment]],
    legend: &'static [HelpLegendRowProps],
    resolver_prose: &'static [&'static str],
    glossary: &'static [&'static [HelpGlossaryEntryProps]],
}

impl HelpContent {
    pub fn workflow(&self) -> &'static [&'static [HelpSegment]] {
        self.workflow
    }

    pub fn legend(&self) -> &'static [HelpLegendRowProps] {
        self.legend
    }

    pub fn resolver_prose(&self) -> &'static [&'static str] {
        self.resolver_prose
    }

    pub fn glossary(&self) -> &'static [&'static [HelpGlossaryEntryProps]] {
        self.glossary
    }
}

/// The guide content, defined once and passed down from the dialog root.
pub const HELP_CONTENT: HelpContent = HelpContent {
    workflow: WORKFLOW,
    legend: LEGEND,
    resolver_prose: RESOLVER_PROSE,
    glossary: GLOSSARY,
};

const WORKFLOW: &[&[HelpSegment]] = &[
    &[HelpSegment::Text {
        content: "On your first visit the editor starts from the Warcraft III default keybinds.",
    }],
    &[HelpSegment::Text {
        content: "When you come back it restores the progress you left, saved in your browser.",
    }],
    &[
        HelpSegment::Text {
            content: "Apply a template ",
        },
        HelpSegment::Icon {
            svg: ICON_TEMPLATES,
        },
        HelpSegment::Text {
            content: " to start from a known setup.",
        },
    ],
    &[
        HelpSegment::Text {
            content: "You might also just import ",
        },
        HelpSegment::Icon { svg: ICON_UPLOAD },
        HelpSegment::Text {
            content: " your own CustomKeys.txt file.",
        },
    ],
    &[
        HelpSegment::Text {
            content: "Open the collisions ",
        },
        HelpSegment::Icon {
            svg: ICON_COLLISIONS,
        },
        HelpSegment::Text {
            content: " page to see every conflict the editor found.",
        },
    ],
    &[
        HelpSegment::Text {
            content: "Click the resolve ",
        },
        HelpSegment::Icon { svg: ICON_RESOLVE },
        HelpSegment::Text {
            content: " button in the toolbar to open the resolver page.",
        },
    ],
    &[HelpSegment::Text {
        content: "Press apply on the resolver page to settle every positional conflict at once.",
    }],
    &[
        HelpSegment::Text {
            content: "Open the grid layout ",
        },
        HelpSegment::Icon { svg: ICON_GRID },
        HelpSegment::Text {
            content: " editor and set a hotkey for each button position.",
        },
    ],
    &[HelpSegment::Text {
        content: "Apply that grid to every unit to clear any remaining hotkey conflicts.",
    }],
    &[HelpSegment::Text {
        content: "Select a unit and drag its abilities between cells to personalize the layout.",
    }],
    &[
        HelpSegment::Text {
            content: "Edit menu and system keys from the system hotkeys ",
        },
        HelpSegment::Icon { svg: ICON_COG },
        HelpSegment::Text {
            content: " dialog.",
        },
    ],
    &[
        HelpSegment::Text {
            content: "Open the preview ",
        },
        HelpSegment::Icon { svg: ICON_PREVIEW },
        HelpSegment::Text {
            content: " to check the text the editor will export.",
        },
    ],
    &[
        HelpSegment::Text { content: "Export " },
        HelpSegment::Icon { svg: ICON_DOWNLOAD },
        HelpSegment::Text {
            content: " your CustomKeys.txt file to where Warcraft III expects.",
        },
    ],
    &[HelpSegment::Text {
        content: "Enjoy your custom keybinds.",
    }],
];

const LEGEND: &[HelpLegendRowProps] = &[
    HelpLegendRowProps {
        icon: ICON_GRID,
        label: "Grid Layout",
        description: "Define a global grid and apply it to every unit.",
    },
    HelpLegendRowProps {
        icon: ICON_COLLISIONS,
        label: "Collisions",
        description: "Review the conflicts the editor found.",
    },
    HelpLegendRowProps {
        icon: ICON_TEMPLATES,
        label: "Templates",
        description: "Apply a prepared keybind set.",
    },
    HelpLegendRowProps {
        icon: ICON_UPLOAD,
        label: "Upload",
        description: "Import a CustomKeys.txt file from your computer.",
    },
    HelpLegendRowProps {
        icon: ICON_COG,
        label: "System Hotkeys",
        description: "Edit the system and menu hotkeys.",
    },
    HelpLegendRowProps {
        icon: ICON_RESOLVE,
        label: "Resolve",
        description: "Display and settle all positional conflicts at once.",
    },
    HelpLegendRowProps {
        icon: ICON_PREVIEW,
        label: "Preview",
        description: "See the text the editor will export.",
    },
    HelpLegendRowProps {
        icon: ICON_DOWNLOAD,
        label: "Export",
        description: "Download your CustomKeys.txt file.",
    },
    HelpLegendRowProps {
        icon: ICON_UNDO,
        label: "Undo",
        description: "Step backward through your changes.",
    },
    HelpLegendRowProps {
        icon: ICON_REDO,
        label: "Redo",
        description: "Step forward through your changes.",
    },
    HelpLegendRowProps {
        icon: ICON_HELP,
        label: "Help",
        description: "Reopen this guide at any time.",
    },
];

const RESOLVER_PROSE: &[&str] = &[
    "The whole clash is modeled as a conflict graph. Every ability is a node, and two of them share an edge whenever a single unit carries both. The tangled clusters those edges form are the islands. Each island is solved on its own. Within one, an anchor is crowned at every contested button, the ability the most units carry, while the losers slide aside. A slide can land on another taken button and spark a fresh fight, so the moves cascade across the row. When a row runs out of room, the stranded ability spills onto another row or swaps with what sits there, and gap pulls tidy the holes left behind. The board is swept round after round until that island settles. Every island found is resolved the same way, so the whole grid lands in one go.",
];

const GLOSSARY: &[&[HelpGlossaryEntryProps]] = &[CONFLICT_KINDS, RESOLVER_PARTS, RESOLVER_MOVES];

const CONFLICT_KINDS: &[HelpGlossaryEntryProps] = &[
    HelpGlossaryEntryProps {
        term: "Cross unit collisions",
        description: "Two or more different units want to place an ability in the same grid cell. A popular ability can pull a whole crowd of units into one knot. This is a position conflict between units.",
    },
    HelpGlossaryEntryProps {
        term: "Intra unit collisions",
        description: "One unit has two or more abilities competing for the same grid cell. This is a position conflict inside a single unit.",
    },
    HelpGlossaryEntryProps {
        term: "Hotkey collisions",
        description: "Two or more abilities on the same unit are bound to the same hotkey letter.",
    },
];

const RESOLVER_PARTS: &[HelpGlossaryEntryProps] = &[
    HelpGlossaryEntryProps {
        term: "Island",
        description: "A connected cluster of abilities that collide on one button. Two abilities are linked when a single unit carries both, and the links can chain from one ability to the next across several units.",
    },
    HelpGlossaryEntryProps {
        term: "Anchor",
        description: "The ability that wins a contested button and stays put. The editor keeps the ability that the most units carry, because moving it would disturb the most cards, so the rarer ability gives way instead.",
    },
    HelpGlossaryEntryProps {
        term: "Mover",
        description: "An ability that steps aside so the anchor can keep the button. It slides one button to the right.",
    },
];

const RESOLVER_MOVES: &[HelpGlossaryEntryProps] = &[
    HelpGlossaryEntryProps {
        term: "Fights",
        description: "Two or more abilities want the same button. The most shared one stays, the others slide to the right.",
    },
    HelpGlossaryEntryProps {
        term: "Gap pulls",
        description: "When a slide leaves an empty button with a filled one further along the same row, the editor pulls that ability back to close the gap and keep the row tidy.",
    },
    HelpGlossaryEntryProps {
        term: "Spills",
        description: "If an ability still has nowhere to go in its own row, the editor rehomes it to a free button in another row.",
    },
    HelpGlossaryEntryProps {
        term: "Swaps",
        description: "Two abilities swap positions in a single move.",
    },
];
