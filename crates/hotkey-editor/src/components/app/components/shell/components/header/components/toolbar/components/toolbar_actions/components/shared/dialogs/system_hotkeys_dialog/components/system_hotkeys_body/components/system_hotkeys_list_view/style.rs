use tw_macro::tw;
classes! {
    base: tw![
        "list-none",
        "m-0",
        "p-0",
        "w-full",
        "max-w-[110rem]",
        "mx-auto",
        "flex",
        "flex-col",
    ],
    mobile: tw![
        "mobile:max-w-full",
        "mobile:[touch-action:pan-y]",
    ],
    tablet: tw![
        "tablet:max-w-full",
        "tablet:[touch-action:pan-y]",
    ],
}
