use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "min-w-0",
        "self-start",
    ],
    mobile: tw![
        "mobile:w-full",
        "mobile:self-stretch",
        "mobile:items-center",
        "mobile:gap-[2cqi]",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:self-stretch",
    ],
}
