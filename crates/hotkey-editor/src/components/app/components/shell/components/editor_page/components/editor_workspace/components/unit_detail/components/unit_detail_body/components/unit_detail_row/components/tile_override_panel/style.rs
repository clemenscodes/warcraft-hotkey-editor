use tw_macro::tw;
// The right column of the unit-detail row: the "Hotkey override" heading and the
// override card (or its empty placeholder). On phones it becomes a sticky bottom
// sheet so the override stays reachable while the grid scrolls.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "min-w-0",
        "self-start",
    ],
    mobile: tw![
        "mobile:w-full",
        "mobile:self-stretch",
        "mobile:items-stretch",
        "mobile:sticky",
        "mobile:bottom-0",
        "mobile:z-[100]",
        "mobile:m-[0_-0.85rem]",
        "mobile:pt-0",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:self-stretch",
        "tablet:pt-0",
    ],
}
