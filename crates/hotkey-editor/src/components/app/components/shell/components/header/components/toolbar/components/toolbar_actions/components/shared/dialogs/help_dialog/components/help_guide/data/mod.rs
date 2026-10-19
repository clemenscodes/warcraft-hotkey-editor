use crate::components::app::components::shell::components::shared::icons::{
    ICON_COG, ICON_COLLISIONS, ICON_DOWNLOAD, ICON_GRID, ICON_HELP, ICON_PREVIEW, ICON_REDO,
    ICON_RESOLVE, ICON_TEMPLATES, ICON_UNDO, ICON_UPLOAD,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelpLegendEntry {
    icon: &'static str,
    label: &'static str,
    description: &'static str,
}

impl HelpLegendEntry {
    pub fn icon(&self) -> &'static str {
        self.icon
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn description(&self) -> &'static str {
        self.description
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HelpGlossaryItem {
    term: &'static str,
    description: &'static str,
}

impl HelpGlossaryItem {
    pub fn term(&self) -> &'static str {
        self.term
    }

    pub fn description(&self) -> &'static str {
        self.description
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelpSegment {
    Text { content: &'static str },
    Icon { svg: &'static str },
}

#[derive(Clone, Copy, PartialEq)]
pub struct HelpContent {
    workflow: &'static [&'static [HelpSegment]],
    legend: &'static [HelpLegendEntry],
    glossary: &'static [&'static [HelpGlossaryItem]],
}

impl HelpContent {
    pub fn workflow(&self) -> &'static [&'static [HelpSegment]] {
        self.workflow
    }

    pub fn legend(&self) -> &'static [HelpLegendEntry] {
        self.legend
    }

    pub fn glossary(&self) -> &'static [&'static [HelpGlossaryItem]] {
        self.glossary
    }
}

pub const HELP_CONTENT: HelpContent = HelpContent {
    workflow: WORKFLOW,
    legend: LEGEND,
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

const LEGEND: &[HelpLegendEntry] = &[
    HelpLegendEntry {
        icon: ICON_GRID,
        label: "Grid Layout",
        description: "Define a global grid and apply it to every unit.",
    },
    HelpLegendEntry {
        icon: ICON_COLLISIONS,
        label: "Collisions",
        description: "Review the conflicts the editor found.",
    },
    HelpLegendEntry {
        icon: ICON_TEMPLATES,
        label: "Templates",
        description: "Apply a prepared keybind set.",
    },
    HelpLegendEntry {
        icon: ICON_UPLOAD,
        label: "Upload",
        description: "Import a CustomKeys.txt file from your computer.",
    },
    HelpLegendEntry {
        icon: ICON_COG,
        label: "System Hotkeys",
        description: "Edit the system and menu hotkeys.",
    },
    HelpLegendEntry {
        icon: ICON_RESOLVE,
        label: "Resolve",
        description: "Display and settle all positional conflicts at once.",
    },
    HelpLegendEntry {
        icon: ICON_PREVIEW,
        label: "Preview",
        description: "See the text the editor will export.",
    },
    HelpLegendEntry {
        icon: ICON_DOWNLOAD,
        label: "Export",
        description: "Download your CustomKeys.txt file.",
    },
    HelpLegendEntry {
        icon: ICON_UNDO,
        label: "Undo",
        description: "Step backward through your changes.",
    },
    HelpLegendEntry {
        icon: ICON_REDO,
        label: "Redo",
        description: "Step forward through your changes.",
    },
    HelpLegendEntry {
        icon: ICON_HELP,
        label: "Help",
        description: "Reopen this guide at any time.",
    },
];

const GLOSSARY: &[&[HelpGlossaryItem]] = &[CONFLICT_KINDS, RESOLVER_MOVES];

const CONFLICT_KINDS: &[HelpGlossaryItem] = &[
    HelpGlossaryItem {
        term: "Cross unit collisions",
        description: "Two or more different units want to place an ability in the same grid cell. A popular ability can pull a whole crowd of units into one knot. This is a position conflict between units.",
    },
    HelpGlossaryItem {
        term: "Intra unit collisions",
        description: "One unit has two or more abilities competing for the same grid cell. This is a position conflict inside a single unit.",
    },
    HelpGlossaryItem {
        term: "Hotkey collisions",
        description: "Two or more abilities on the same unit are bound to the same hotkey letter.",
    },
];

const RESOLVER_MOVES: &[HelpGlossaryItem] = &[
    HelpGlossaryItem {
        term: "Fights",
        description: "Two or more abilities want the same button. The most shared one stays, the others slide to the right.",
    },
    HelpGlossaryItem {
        term: "Gap pulls",
        description: "When a slide leaves an empty button with a filled one further along the same row, the editor pulls that ability back to close the gap and keep the row tidy.",
    },
    HelpGlossaryItem {
        term: "Spills",
        description: "If an ability still has nowhere to go in its own row, the editor rehomes it to a free button in another row.",
    },
    HelpGlossaryItem {
        term: "Swaps",
        description: "Two abilities swap positions in a single move.",
    },
];
