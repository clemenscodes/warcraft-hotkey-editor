use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-[1.2rem]",
        "flex-1",
        "min-w-0",
    ],
    mobile: tw!["mobile:flex-none"],
    tablet: tw!["tablet:flex-none"],
}
