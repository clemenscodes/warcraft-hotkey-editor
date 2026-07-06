use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-stretch",
        "gap-10",
        "grow",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:gap-[0.6rem]",
    ],
}
