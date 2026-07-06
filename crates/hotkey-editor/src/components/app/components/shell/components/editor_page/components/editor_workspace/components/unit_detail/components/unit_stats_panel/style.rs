use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(2,minmax(0,1fr))]",
        "[grid-template-areas:'vitality_attributes'_'combat_defense']",
        "items-stretch",
        "gap-x-[2rem]",
        "gap-y-[2.5rem]",
        "mt-[2.5rem]",
        "mb-[0.75rem]",
        "p-[1.4rem_1.75rem]",
        "bg-warcraft-bg-base/55",
        "border",
        "border-warcraft-blue-deep",
        "rounded-[8px]",
    ],
    mobile: tw![
        "mobile:grid-cols-[minmax(0,1fr)]",
        "mobile:[grid-template-areas:'vitality'_'attributes'_'combat'_'defense']",
        "mobile:gap-5",
        "mobile:p-5",
    ],
    tablet: tw![
        "tablet:gap-y-[1.75rem]",
        "tablet:p-[1.5rem_1.75rem]",
    ],
}
