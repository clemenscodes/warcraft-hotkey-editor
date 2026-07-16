use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "absolute",
        "right-4",
        "top-1/2",
        "-translate-y-1/2",
        "w-10",
        "h-10",
    ],
    mobile: tw![
        "mobile:right-2",
    ],
    tablet: tw![
        "tablet:right-2",
    ],
}
