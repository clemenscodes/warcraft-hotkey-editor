use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-2",
        "[grid-template-areas:'vitality_attributes'_'combat_defense']",
        "items-stretch",
        "gap-x-[2.92cqi]",
        "gap-y-[3.65cqi]",
        "py-[2.19cqi]",
        "px-[2.55cqi]",
        "bg-warcraft-bg-base/55",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
    ],
    mobile: tw![
        "mobile:flex",
        "mobile:flex-col",
        "mobile:gap-[calc(1.82cqi+var(--stats-spread,0)*1px)]",
        "mobile:p-[1.82cqi]",
    ],
    tablet: tw![
        "tablet:gap-y-[2.55cqi]",
        "tablet:py-[2.19cqi]",
        "tablet:px-[2.55cqi]",
    ],
}
