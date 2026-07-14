use tw_macro::tw;

classes! {
    base: tw![
        "grid",
        "grid-cols-[minmax(0,1fr)_auto_auto]",
        "items-center",
        "gap-x-3.5",
        "h-20",
    ],
    mobile: tw![
        "mobile:h-[4.6rem]",
    ],
    tablet: tw![
        "tablet:h-[4.6rem]",
    ],
}
