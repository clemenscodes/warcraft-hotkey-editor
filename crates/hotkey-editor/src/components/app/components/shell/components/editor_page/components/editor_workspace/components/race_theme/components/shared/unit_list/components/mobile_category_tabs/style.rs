use tw_macro::tw;
// The mobile/tablet category tab row. Hidden on the sidebar; a full-width, single-row
// flex strip of tabs on small screens.

classes! {
    base: tw!["hidden"],
    mobile: tw![
        "mobile:flex",
        "mobile:flex-row",
        "mobile:flex-nowrap",
        "mobile:gap-1.5",
        "mobile:w-full",
        "mobile:min-w-0",
        "mobile:p-0",
    ],
    tablet: tw![
        "tablet:flex",
        "tablet:flex-row",
        "tablet:flex-nowrap",
        "tablet:gap-1.5",
        "tablet:w-full",
        "tablet:min-w-0",
        "tablet:p-0",
    ],
}
