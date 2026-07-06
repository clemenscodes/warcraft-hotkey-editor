use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-row",
        "flex-nowrap",
        "items-start",
        "gap-8",
    ],
    mobile: tw!["mobile:gap-[8px]"],
    tablet: tw!["tablet:gap-[8px]"],
}
