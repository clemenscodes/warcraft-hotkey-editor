use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "items-start",
        "gap-[1.25em]",
        "min-w-0",
    ],
    // A phone has no width to spare beside the portrait, so the description
    // flows around it instead of standing in a narrow column next to it.
    mobile: tw![
        "mobile:flow-root",
    ],
}
