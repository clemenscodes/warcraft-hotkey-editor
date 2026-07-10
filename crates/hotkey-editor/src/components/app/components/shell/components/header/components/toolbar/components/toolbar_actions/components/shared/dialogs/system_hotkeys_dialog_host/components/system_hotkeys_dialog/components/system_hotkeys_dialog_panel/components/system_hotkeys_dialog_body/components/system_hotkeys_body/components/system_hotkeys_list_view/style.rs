use tw_macro::tw;
classes! {
    base: tw![
        "list-none",
        "p-0",
        "w-full",
        "max-w-440",
        "self-center",
        "flex",
        "flex-col",
    ],
    mobile: tw![
        "mobile:max-w-full",
        "mobile:[touch-action:pan-y]",
    ],
    tablet: tw![
        "tablet:max-w-full",
        "tablet:[touch-action:pan-y]",
    ],
}
