use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "min-w-0",
        "self-start",
    ],
    mobile: tw![
        "mobile:self-stretch",
        "mobile:items-stretch",
        "mobile:sticky",
        "mobile:bottom-0",
        "mobile:z-100",
        "mobile:-left-[0.85rem]",
        "mobile:w-[calc(100%+1.7rem)]",
        "mobile:pt-0",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:self-stretch",
        "tablet:pt-0",
    ],
}
