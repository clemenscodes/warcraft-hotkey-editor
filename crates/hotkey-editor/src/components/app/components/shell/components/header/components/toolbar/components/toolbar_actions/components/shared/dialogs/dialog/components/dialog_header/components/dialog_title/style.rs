use tw_macro::tw;
classes! {
    base: tw![
        "m-0",
        "text-3xl",
        "leading-none",
    ],
    mobile: tw![
        "mobile:min-w-0",
        "mobile:max-w-full",
        "mobile:overflow-hidden",
        "mobile:text-sm",
        "mobile:leading-none",
        "mobile:text-ellipsis",
        "mobile:whitespace-nowrap",
    ],
    tablet: tw![
        "tablet:min-w-0",
        "tablet:max-w-full",
        "tablet:overflow-hidden",
        "tablet:text-sm",
        "tablet:leading-none",
        "tablet:text-ellipsis",
        "tablet:whitespace-nowrap",
    ],
}
