use tw_macro::tw;

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
