use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "items-center",
        "gap-8",
        "w-full",
    ],
    mobile: tw![
        "mobile:gap-3.5",
        "mobile:px-3",
        "mobile:my-auto",
    ],
    tablet: tw![
        "tablet:gap-3.5",
    ],
}
