use tw_macro::tw;
classes! {
    base: tw!["rotate-180"],
    mobile: tw![
        "mobile:flex-none",
        "mobile:text-[0.9em]",
        "mobile:leading-none",
        "mobile:transition-[transform]", "mobile:duration-slow",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:text-[0.9em]",
        "tablet:leading-none",
        "tablet:transition-[transform]", "tablet:duration-slow",
    ],
}
