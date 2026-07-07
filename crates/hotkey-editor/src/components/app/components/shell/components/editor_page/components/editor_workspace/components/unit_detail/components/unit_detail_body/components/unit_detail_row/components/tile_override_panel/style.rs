use tw_macro::tw;
// The right column of the unit-detail row: the "Hotkey override" heading and the
// override card (or its empty placeholder). On phones it becomes a sticky bottom
// sheet so the override stays reachable while the grid scrolls. It intentionally
// breaks out of the unit-detail card's horizontal padding on mobile, so it is
// widened and shifted (not margined) rather than inset like its siblings.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "min-w-0",
        "self-start",
    ],
    mobile: tw![
        "mobile:self-stretch",
        "mobile:items-stretch",
        "mobile:sticky",
        "mobile:bottom-0",
        "mobile:z-100",
        "mobile:-left-[0.85rem]",
        "mobile:w-[calc(100%+1.7rem)]",
        "mobile:pt-0",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:self-stretch",
        "tablet:pt-0",
    ],
}
