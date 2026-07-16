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
        "mobile:flex-1",
        "mobile:min-h-0",
        "mobile:items-center",
        "mobile:gap-[2cqi]",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:self-stretch",
    ],
}
