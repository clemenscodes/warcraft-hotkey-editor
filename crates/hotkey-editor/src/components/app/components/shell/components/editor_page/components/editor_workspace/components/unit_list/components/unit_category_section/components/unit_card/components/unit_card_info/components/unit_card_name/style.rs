use tw_macro::tw;
classes! {
    base: tw![
        "text-base",
        "leading-title",
        "pb-[0.1rem]",
        "overflow-hidden",
        "text-ellipsis",
        "whitespace-nowrap",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:block",
        "mobile:w-full",
        "mobile:text-xs",
        "mobile:leading-title",
        "mobile:data-[selected=true]:text-white",
    ],
    tablet: tw![
        "tablet:block",
        "tablet:w-full",
        "tablet:text-xs",
        "tablet:leading-title",
        "tablet:data-[selected=true]:text-white",
    ],
    desktop: tw!["desktop:text-lg"],
    qhd: tw!["qhd:text-lg"],
    uhd: tw!["uhd:text-lg"],
}
