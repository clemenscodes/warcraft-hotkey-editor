use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "items-stretch",
        "px-0",
        "pt-0",
        "flex-none",
        "min-h-0",
        "gap-5",
    ],
    mobile: tw![
        "mobile:grid",
        "mobile:grid-cols-[minmax(0,1fr)]",
        "mobile:gap-6",
        "mobile:items-start",
    ],
    tablet: tw![
        "tablet:grid",
        "tablet:grid-cols-[minmax(0,1fr)]",
        "tablet:gap-6",
        "tablet:items-start",
    ],
}
