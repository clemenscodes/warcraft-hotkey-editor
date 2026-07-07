use tw_macro::tw;
// The name-and-id column of the override header; centered with a touch floor on the
// mobile panel so the row height stays stable.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-start",
        "gap-1.5",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:gap-0.5",
        "mobile:min-h-11",
        "mobile:justify-center",
        "mobile:text-left",
    ],
    tablet: tw![
        "tablet:gap-0.5",
        "tablet:min-h-11",
        "tablet:justify-center",
        "tablet:text-left",
    ],
}
