use tw_macro::tw;
classes! {
    base: tw![
        "text-[1.05rem]",
        "leading-[1.25]",
        "pb-[0.1rem]",
        "overflow-hidden",
        "text-ellipsis",
        "whitespace-nowrap",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:block",
        "mobile:w-full",
        "mobile:text-[11px]",
        "mobile:leading-[1.2]",
        "mobile:data-[selected=true]:text-white",
    ],
    tablet: tw![
        "tablet:block",
        "tablet:w-full",
        "tablet:text-[11px]",
        "tablet:leading-[1.2]",
        "tablet:data-[selected=true]:text-white",
    ],
    desktop: tw!["desktop:text-[1.35rem]"],
    qhd: tw!["qhd:text-[1.35rem]"],
    uhd: tw!["uhd:text-[1.35rem]"],
}
