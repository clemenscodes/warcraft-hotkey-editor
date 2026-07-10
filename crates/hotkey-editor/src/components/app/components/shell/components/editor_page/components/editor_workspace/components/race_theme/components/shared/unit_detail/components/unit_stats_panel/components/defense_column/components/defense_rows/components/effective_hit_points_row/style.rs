use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "gap-2",
        "text-xl",
        "leading-title",
        "text-shadow-drop",
        "min-w-0",
        "flex-[1_1_auto]",
    ],
    mobile: tw!["mobile:text-2xl", "mobile:leading-heading"],
}
