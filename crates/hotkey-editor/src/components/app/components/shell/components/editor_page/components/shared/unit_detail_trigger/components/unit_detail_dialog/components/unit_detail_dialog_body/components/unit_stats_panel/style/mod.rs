use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-2",
        "[grid-template-areas:'vitality_attributes'_'combat_defense']",
        "items-stretch",
        "gap-x-[2.5em]",
        "gap-y-[1.5em]",
        "p-[1.25em]",
        "bg-warcraft-bg-base/55",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
    ],
    mobile: tw![
        "mobile:flex",
        "mobile:flex-col",
        "mobile:gap-[1.82cqi]",
        "mobile:p-[1.82cqi]",
    ],
    // A unit without attributes leaves the right column's top empty, so the
    // defense section takes the whole column instead.
    tablet: tw![
        "tablet:has-[>.attributes-stats-host:empty]:[grid-template-areas:'vitality_defense'_'combat_defense']",
    ],
    laptop: tw![
        "laptop:has-[>.attributes-stats-host:empty]:[grid-template-areas:'vitality_defense'_'combat_defense']",
    ],
    desktop: tw![
        "desktop:has-[>.attributes-stats-host:empty]:[grid-template-areas:'vitality_defense'_'combat_defense']",
    ],
    qhd: tw![
        "qhd:has-[>.attributes-stats-host:empty]:[grid-template-areas:'vitality_defense'_'combat_defense']",
    ],
    uhd: tw![
        "uhd:has-[>.attributes-stats-host:empty]:[grid-template-areas:'vitality_defense'_'combat_defense']",
    ],
}
