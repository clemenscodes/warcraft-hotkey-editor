use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex-1",
        "min-h-0",
        "flex",
        "flex-col",
        "gap-6",
        "pt-10",
        "px-12",
        "pb-10",
        "overflow-y-auto",
    ],
    mobile: tw![
        "mobile:pt-5",
        "mobile:px-4",
        "mobile:pb-6",
    ],
}
