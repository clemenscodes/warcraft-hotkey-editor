use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-2",
        "gap-9",
        "w-full",
    ],
    mobile: tw![
        "mobile:grid-cols-1",
        "mobile:gap-2.5",
    ],
    tablet: tw![
        "tablet:grid-cols-1",
        "tablet:gap-2.5",
    ],
}
