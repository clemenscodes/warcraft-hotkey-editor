use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-2",
        "[grid-template-areas:'vitality_attributes'_'combat_defense']",
        "items-stretch",
        "gap-x-8",
        "gap-y-10",
        "py-6",
        "px-7",
        "bg-warcraft-bg-base/55",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
    ],
    mobile: tw![
        "mobile:grid-cols-[minmax(0,1fr)]",
        "mobile:[grid-template-areas:'vitality'_'attributes'_'combat'_'defense']",
        "mobile:gap-5",
        "mobile:p-5",
    ],
    tablet: tw![
        "tablet:gap-y-7",
        "tablet:py-6",
        "tablet:px-7",
    ],
}
