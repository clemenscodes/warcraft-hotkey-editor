use tw_macro::tw;
// The inner track laying out the category sections: a vertical stack on the sidebar,
// a horizontal fixed-height carousel on small screens.

classes! {
    base: tw!["flex", "flex-col", "gap-2"],
    mobile: tw![
        "mobile:flex-row",
        "mobile:flex-nowrap",
        "mobile:w-max",
        "mobile:min-w-full",
        "mobile:items-stretch",
        "mobile:h-full",
        "mobile:px-1.5",
        "mobile:py-0",
    ],
    tablet: tw![
        "tablet:flex-row",
        "tablet:flex-nowrap",
        "tablet:w-max",
        "tablet:min-w-full",
        "tablet:items-stretch",
        "tablet:h-full",
        "tablet:px-1.5",
        "tablet:py-0",
    ],
}
