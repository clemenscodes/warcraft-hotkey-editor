use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "gap-3",
        "pt-2",
        "flex-[1_1_0]",
        "min-w-0",
        "self-stretch",
        "@container",
    ],
    mobile: tw![
        "mobile:gap-[1cqi]",
        "mobile:pt-0",
    ],
}
