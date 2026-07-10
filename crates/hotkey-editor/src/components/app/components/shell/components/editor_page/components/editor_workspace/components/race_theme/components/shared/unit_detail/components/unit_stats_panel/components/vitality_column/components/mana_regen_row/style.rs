use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "justify-between",
        "gap-2",
        "text-xl",
        "leading-title",
        "text-shadow-drop",
        "min-w-0",
        "-translate-y-[0.2rem]",
        "pl-5",
    ],
    mobile: tw!["mobile:text-2xl", "mobile:leading-heading"],
}
