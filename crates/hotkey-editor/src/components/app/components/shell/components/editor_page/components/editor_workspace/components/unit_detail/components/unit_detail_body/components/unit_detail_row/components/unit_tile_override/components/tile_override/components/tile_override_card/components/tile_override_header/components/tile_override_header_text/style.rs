use tw_macro::tw;
// The name-and-id column of the override header; centered with a touch floor on the
// mobile panel so the row height stays stable.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-start",
        "gap-[0.4rem]",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:gap-[2px]",
        "mobile:min-h-[44px]",
        "mobile:justify-center",
        "mobile:text-left",
    ],
    tablet: tw![
        "tablet:gap-[2px]",
        "tablet:min-h-[44px]",
        "tablet:justify-center",
        "tablet:text-left",
    ],
}
