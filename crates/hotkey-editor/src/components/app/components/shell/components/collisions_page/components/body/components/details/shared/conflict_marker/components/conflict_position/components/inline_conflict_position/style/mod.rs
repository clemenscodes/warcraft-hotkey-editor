use tw_macro::tw;
classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "*:h-15",
        "*:w-20",
        "self-start",
        "h-18",
    ],
    mobile: tw![
        "mobile:*:w-26",
        "mobile:*:h-19.5",
        "mobile:h-auto",
    ],
}
